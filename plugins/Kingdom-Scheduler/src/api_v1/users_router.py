from fastapi import APIRouter, Body, Depends
from models import rr_models
from core.types import BoxedResult
from dataspace.dataspace import Dataspace
from core import middleware
router = APIRouter(
    prefix='/api/v1'
)

dspc = Dataspace()


from fastapi import Request, HTTPException
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials

class JWTBearer(HTTPBearer):
    def __init__(self, auto_error: bool = True):
        super(JWTBearer, self).__init__(auto_error=auto_error)

    async def __call__(self, request: Request):
        credentials: HTTPAuthorizationCredentials = await super(JWTBearer, self).__call__(request)
        if credentials:
            if not credentials.scheme == "Bearer":
                raise HTTPException(status_code=403, detail="Invalid authentication scheme.")
            if not await self.verify_jwt(credentials.credentials):
                raise HTTPException(status_code=403, detail="Invalid token or expired token.")
            return credentials.credentials
        else:
            raise HTTPException(status_code=403, detail="Invalid authorization code.")

    async def verify_jwt(self, jwtoken: str) -> bool:
        isTokenValid: bool = False

        try:
            payload = await middleware.decode_jwt(jwtoken)
        except:
            payload = None
        if payload and payload['role'] in ['admin', 'manager']:
            isTokenValid = True

        return isTokenValid



def check_user(data: rr_models.UserResponse):
        for user in dspc.getAllUsers().users:
            if user.username == data.username:
                return (True, user.role)
        return (False, None)

@router.post('/change_role', dependencies=[Depends(JWTBearer())])
async def change_role(user_id: str, role: str) -> BoxedResult[rr_models.Result]:
    return dspc.changeRole(user_id, role)    

@router.get('/users')
async def get_users() -> BoxedResult[rr_models.UsersResponse]:
    return dspc.getAllUsers()

@router.get('/students')
async def get_students() -> BoxedResult[rr_models.UsersResponse]:
    return dspc.getByRole(role='student')

@router.get('/teachers')
async def get_teachers() -> BoxedResult[rr_models.UsersResponse]:
    return dspc.getByRole(role='teacher')

@router.post('/sign_up')
async def add_user(user: rr_models.UserRequest) -> dict:
    dspc.addUser(user)
    return await middleware.sign_jwt(user.username, 'student')

@router.post('/sing_in')
async def user_login(user: rr_models.SignInRequestForm = Body(...)):
    checked = check_user(user)
    if checked[0]:
        return await middleware.sign_jwt(user.username, checked[1])
    return {
        'error': 'wrong login details.'
    }

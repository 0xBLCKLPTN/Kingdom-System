from fastapi import APIRouter
import jwt
from jwt.exceptions import InvalidTokenError
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm


router = APIRouter(
    prefix='/api/v1/auth'
)

@router.get('/getMe')
async def get_me() -> object:
    pass

@router.post('/sign_in')
async def sign_in() -> object:
    pass

@router.post('/signUp')
async def sign_up() -> object:
    pass

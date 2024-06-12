import time
import jwt
from typing import Dict

JWT_SECRET = "1611fef0882f9d70b645cbd82c09f3b61a1a3820c8e7b4f9e0e4cee35ef64396"
JWT_ALGORITHM = "HS256"

#pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


async def sign_jwt(user_id: str, user_role: str) -> Dict[str, str]:
    payload = {
        'user_id': user_id,
        'expires': time.time() + 600,
        'role': user_role 
    }
    token = jwt.encode(payload, JWT_SECRET, algorithm=JWT_ALGORITHM)
    return await token_response(token)

async def token_response(token: str):
    return {
        'access_token': token
    }

async def decode_jwt(token: str) -> dict:
    try:
        decoded_token = jwt.decode(token, JWT_SECRET, algorithms=[JWT_ALGORITHM])
        return decoded_token if decoded_token['expires'] >= time.time() else None
    except:
        return {}



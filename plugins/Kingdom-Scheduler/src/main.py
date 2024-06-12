from fastapi import FastAPI
from models import rr_models
from api_v1 import users_router, groups_router, auth_router, schedule_router

app = FastAPI()

app.include_router(users_router.router)
app.include_router(groups_router.router)
app.include_router(auth_router.router)
app.include_router(schedule_router.router)

@app.get('/health')
async def get_health() -> rr_models.Health:
    return rr_models.Health(health=100) 

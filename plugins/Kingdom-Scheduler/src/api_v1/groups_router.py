from fastapi import APIRouter
from models import rr_models
from core.types import BoxedResult

from dataspace.dataspace import Dataspace

router = APIRouter(
    prefix='/api/v1'
)

dspc = Dataspace()


@router.get('/groups')
async def get_groups() -> BoxedResult[rr_models.GroupsResponse]:
    return dspc.getAllGroups()

@router.post('/group')
async def add_group(group: rr_models.GroupRequest) -> BoxedResult[rr_models.Result]:
    return dspc.addGroup(group)

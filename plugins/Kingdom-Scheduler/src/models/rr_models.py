from core.types import *
from pydantic import BaseModel

class Health(BaseModel):
    health: int

class Result(BaseModel):
    result: Union[str, None]

class UserNames(BaseModel):
    firstName: str
    middleName: Union[str, None]
    lastName: str

class UserResponse(BaseModel):
    id: str
    role: str
    username: str
    names: UserNames

class UsersResponse(BaseModel):
    users: List[UserResponse]


class UserRequest(BaseModel):
    username: str
    names: UserNames
    password: str

class GroupResponse(BaseModel):
    id: str
    name: str
    course: int
    budget: int

class GroupsResponse(BaseModel):
    groups: List[GroupResponse]

class GroupRequest(BaseModel):
    name: str
    course: int
    budget: int


class SignInRequestForm(BaseModel):
    username: str
    password: str


class TokensResponse(BaseModel):
    auth_token: str
    refresh_token: str

class LessonResponse(BaseModel):
    teacher_id: str
    lesson_id: str
    lesson_number: int
    cabinet_number: str

class LessonRequest(BaseModel):
    name: str


class RLessonResponse(BaseModel):
    id: str
    name: str

class CreateSchedule(BaseModel):
    date: str
    group_id: str
    lessons: List[LessonResponse]

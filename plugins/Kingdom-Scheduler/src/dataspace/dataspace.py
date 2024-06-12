from models import database_models as dm
from models import rr_models
from core.types import *
from core.db import create_database 
from core import print_ex
from motor.motor_asyncio import AsyncIOMotorClient
from pymongo.server_api import ServerApi
import asyncio

session = create_database()

def create_mongoclient():
    return AsyncIOMotorClient('mongodb://root:root@localhost:27017')

pyclient = create_mongoclient()

class Dataspace:
    def __init__(self) -> Self:
        self.ex_prefix = '[DATASPACE]=>'

    def __convert_user_from_db(self, obj: object) -> BoxedResult[rr_models.UserResponse]:
        return rr_models.UserResponse(
                                id=str(obj.id),
                                role=obj.role,
                                username=obj.username,
                                names=rr_models.UserNames(
                                    firstName = obj.firstName,
                                    middleName = obj.middleName,
                                    lastName = obj.lastName,
                                )
                            )
    def __convert_groups_from_db(self, obj: object) -> BoxedResult[rr_models.GroupsResponse]:
        return rr_models.GroupResponse(
            id = str(obj.id),
            name = obj.name,
            course = obj.course,
            budget = obj.budget,
        )

    def __convert_lesson_from_db(self, obj: object) -> BoxedResult[rr_models.RLessonResponse]:
        return rr_models.RLessonResponse(
            id = str(obj.id),
            name = obj.name
        )
    
    def getAllUsers(self) -> BoxedResult[rr_models.UsersResponse]:
        result = None
        try:
            return rr_models.UsersResponse(
                            users=[self.__convert_user_from_db(user) for user in session.query(dm.UsersDBModel).all()]
                    )
        except Exception as ex:
            return print_ex(self.ex_prefix, ex)

    def getByRole(self, role: str) -> BoxedResult[rr_models.UsersResponse]:
        result = None
        if not role:
            result
        try:
            return rr_models.UsersResponse(
                users=[self.__convert_user_from_db(user) for user in session.query(dm.UsersDBModel).filter(dm.UsersDBModel.role == role)]
            )
        except Exception as ex:
            return print_ex(self.ex_prefix, ex)

    def addUser(self, user: rr_models.UserRequest) -> BoxedResult[rr_models.Result]:
        user = dm.UsersDBModel(
                        firstName=user.names.firstName,
                        middleName=user.names.middleName,
                        lastName=user.names.lastName,
                        username=user.username,
                        hashed_password=user.password)
        session.add(user)
        session.commit()
        return rr_models.Result(result='User Created!')

    def getAllGroups(self) -> BoxedResult[rr_models.GroupsResponse]:
        result = None
        try:
            return rr_models.GroupsResponse(
                groups = [self.__convert_groups_from_db(group) for group in session.query(dm.GroupsDBModel).all()]
            )
        except Exception as ex:
            return print_ex(self.ex_prefix, ex)
    
    def addGroup(self, group: rr_models.GroupRequest) -> BoxedResult[rr_models.Result]:
        group = dm.GroupsDBModel(
            name = group.name,
            course = group.course,
            budget = group.budget,
        )
        session.add(group)
        session.commit()
        return rr_models.Result(result='Group Created!')
    
    def changeRole(self, user_id: str, role: str) -> BoxedResult[rr_models.Result]:
        session.query(dm.UsersDBModel).filter(dm.UsersDBModel.id == user_id).update({'role': role})
        session.commit()
        return rr_models.Result(result='role changed!') 

    def getTeacherById(self, teacher_id: str) -> BoxedResult[rr_models.UserResponse]:
        return session.query(dm.UsersDBModel).filter(dm.UsersDBModel.id == teacher_id).one()

    def getGroupById(self, group_id: str) -> BoxedResult[rr_models.GroupResponse]:
        return session.query(dm.GroupsDBModel).filter(dm.GroupsDBModel.id == group_id).one()

    def getLessonById(self, lesson_id: str) -> BoxedResult[rr_models.LessonResponse]:
        return session.query(dm.LessonsDBModel).filter(dm.LessonsDBModel.id == lesson_id).one()

    def addLesson(self, lesson: rr_models.LessonRequest) -> BoxedResult[rr_models.Result]:
        session.add(dm.LessonsDBModel(name=lesson.name))
        session.commit()
        return rr_models.Result(result='lesson created!')

    def getAllLessons(self) -> BoxedResult[List[rr_models.RLessonResponse]]:
        lessons = []
        for lesson in session.query(dm.LessonsDBModel).all():
            lessons.append(self.__convert_lesson_from_db(lesson))
        return lessons
            

import json
from bson import ObjectId, json_util

class JSONEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, ObjectId):
            return str(o)
        return json.JSONEncoder.default(self, o)




def parse_json(data):
    return json.loads(json_util.dumps(data))


class MongoSpace:
    def __init__(self):
        self.prefix = "[ MONGOSPACE ]=> "
        self.db = pyclient.schedules
        

        
    async def add_schedule(self, schedule: dict):
        self.collection = self.db[schedule['date']]
        await self.collection.insert_one(schedule)
        print(await self.collection.find_one())

    async def get_schedule(self, date: str):
        self.collection = self.db[date]
        data = await self.collection.find().to_list(length=None)
        return parse_json(data)

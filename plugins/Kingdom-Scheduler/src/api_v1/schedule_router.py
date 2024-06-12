from fastapi import APIRouter, Depends
from dataspace.dataspace import MongoSpace, Dataspace
from models import rr_models
from core.types import *
import json
from .users_router import JWTBearer

router = APIRouter(
    prefix='/api/v1/schedule'
)

ms = MongoSpace()
dspc = Dataspace()

def create_teacher_dict(teacher_id: str) -> dict:
    teacher = dspc.getTeacherById(teacher_id)
    return {
        'teacher_id': str(teacher.id),
        'full_name': teacher.firstName +' '+ teacher.middleName + ' '+teacher.lastName
    }

def create_lesson_dict(lesson_id: str) -> dict:
    lesson = dspc.getLessonById(lesson_id)
    return {
        'id': str(lesson.id),
        'name': lesson.name
    }

def create_group_dict(group_id: str) -> dict:
    group = dspc.getGroupById(group_id)
    return {
        'id': str(group.id),
        'name': group.name,
        'course': group.course,
        'budget': group.budget
    }

@router.get('/get_schedule', )
async def get_schedule(date: str) -> object:
    return await ms.get_schedule(date)

@router.post('/create_schedule', dependencies=[Depends(JWTBearer())])
async def create_schedule(schedule: rr_models.CreateSchedule) -> BoxedResult[rr_models.Result]:
    schedule = {
        'date': schedule.date,
        'group': create_group_dict(schedule.group_id),
        'lessons': [
            {
                'teacher': create_teacher_dict(i.teacher_id),
                'lesson': create_lesson_dict(i.lesson_id),
                'lesson_number': i.lesson_number,
                'cabinet_number': i.cabinet_number
            } for i in schedule.lessons]
    }
    print(schedule)
    await ms.add_schedule(schedule)
    return rr_models.Result(result='schedule created!')

@router.post('/add-lesson', dependencies=[Depends(JWTBearer())])
async def add_lesson(lesson: rr_models.LessonRequest) -> BoxedResult[rr_models.Result]:
    return dspc.addLesson(lesson)

@router.get('/lessons')
async def lessons() -> BoxedResult[List[rr_models.RLessonResponse]]:
    return dspc.getAllLessons()

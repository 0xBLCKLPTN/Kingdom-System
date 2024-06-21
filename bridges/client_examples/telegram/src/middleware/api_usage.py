import requests
from datetime import datetime

URL = 'http://127.0.0.1:8000/'

def get_teachers():
    response = requests.get(URL + '/api/v1/users').json()['users']
    
    text = "СПИСОК УЧИТЕЛЕЙ:\n"

    for i in response:
        text += '------------\n'
        text += f'ID: {i['id']}\n'
        text += f'Имя пользователя: {i['username']}\n'
        text += f'ФИО: {i['names']['lastName']} {i['names']['firstName']} {i['names']['middleName']}\n\n'

    return text
    
def get_lessons():
    response = requests.get(URL + '/api/v1/schedule/lessons').json()

    text = "СПИСОК ПРЕДМЕТОВ:\n"

    for i in response:
        text += '------------\n'
        text += f'ID: {i['id']}\n'
        text += f'Наименование: {i['name']}\n\n'
    return text

def get_today_schedule():
    response = requests.get(URL + f'/api/v1/schedule/get_schedule?date={datetime.today().strftime('%d.%m.%Y')}').json()
    text = f"РАСПИСАНИЕ НА СЕГОДНЯ: {response[0]['date']}\n"
    
    for i in response:
        text += '-------------\n'
        text += f'Группа: {i['group']['course']}{i['group']['name']}{i['group']['budget']}\n'
        lessons_text = ''
        for k in i['lessons']:
            lessons_text += f'{k['lesson_number']}-{k['lesson']['name']}\n'
            lessons_text += f'Кабинет: {k['cabinet_number']}\n'
        text += lessons_text
        text += '\n\n'

    return text



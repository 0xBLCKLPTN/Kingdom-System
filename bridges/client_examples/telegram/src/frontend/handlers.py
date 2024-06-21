from dispatcher import dp, bot
from aiogram import types

from aiogram.filters import CommandStart
from aiogram.types import Message

from aiogram.utils.keyboard import InlineKeyboardBuilder, InlineKeyboardButton
from aiogram import F

from middleware import api_usage

def to_main_kb():
    kb = [
        [types.KeyboardButton(text="На главную")],
        
    ]
    keyboard = types.ReplyKeyboardMarkup(keyboard=kb)
    return keyboard
    
def generate_default_keyboard():
    builder = InlineKeyboardBuilder()
    builder.add(
        types.InlineKeyboardButton(
            text="Список учителей",
            callback_data="list_of_teachers"
        )
    )
    builder.add(
        types.InlineKeyboardButton(
            text='Список предметов',
            callback_data='list_of_lessons'
        )
    )
    builder.add(
        types.InlineKeyboardButton(
            text='Расписание на сегодня',
            callback_data='today_schedule'
        )
    )
    builder.adjust(1, 3)
    return builder


@dp.message(CommandStart())
async def command_start_handler(message: Message) -> None:
    await message.answer('Привет! Меня зовут Ракута!', reply_markup=generate_default_keyboard().as_markup())


@dp.message(F.text.lower() == 'на главную')
async def message_handler(message: Message) -> None:
    await message.answer('Выберите действие', reply_markup=generate_default_keyboard().as_markup())

@dp.callback_query(F.data == 'list_of_teachers')
async def send_list_of_teachers(callback: types.CallbackQuery):
    await callback.message.answer(api_usage.get_teachers(), reply_markup=to_main_kb())

@dp.callback_query(F.data == 'list_of_lessons')
async def send_list_of_lessons(callback: types.CallbackQuery):
    await callback.message.answer(api_usage.get_lessons(), reply_markup=to_main_kb())

@dp.callback_query(F.data == 'today_schedule')
async def send_today_schedule(callback: types.CallbackQuery):
    await callback.message.answer(api_usage.get_today_schedule(), reply_markup=to_main_kb())

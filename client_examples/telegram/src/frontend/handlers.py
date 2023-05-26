from dispatcher import dp, bot
from aiogram import types

@dp.message_handler(commands=["start"])
async def proccess_start_command(message: types.Message):
    await message.reply("Hello!")

@dp.message_handler(commands=["users"])
async def proccess_get_schedules(message: types.Message):
    await message.reply("HIIII")

from aiogram import Bot, types
from aiogram.dispatcher import Dispatcher
from aiogram.utils import executor
from core.config import settings
from aiogram.contrib.fsm_storage.memory import MemoryStorage
from aiogram.contrib.middlewares.logging import LoggingMiddleware

# Хранение данных в оперативной памяти
storage = MemoryStorage()

# Инициализация aiogram бота.
bot = Bot(token=settings.BOT_TOKEN)

# Инициализация диспатчера
dp = Dispatcher(bot, storage=storage)

# Добавление логирования в диспатчер
dp.middleware.setup(LoggingMiddleware())

from dispatcher import dp, bot
from frontend import handlers
import asyncio

async def main():
    await dp.start_polling(bot)

if __name__ == '__main__':
    asyncio.run(main())

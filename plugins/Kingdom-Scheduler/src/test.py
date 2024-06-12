import asyncio
from motor.motor_asyncio import AsyncIOMotorClient


async def ping_server():
    uri = "mongodb://root:root@localhost:27017"

    client = AsyncIOMotorClient(uri)

    try:
        await client.admin.command('ping')
        print("Pinged your deployment")
    except Exception as ex:
        print(ex)

asyncio.run(ping_server())

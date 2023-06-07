from fastapi import FastAPI, File, UploadFile
from fastapi.responses import FileResponse
from typing_extensions import Annotated
import uvicorn
from crud import get_database
import uuid
from pathlib import Path
import os


def create_folder(user_id: str) -> None:
    if user_id not in os.listdir('files/'):
        Path(f"files/{user_id}").mkdir(parents=True, exist_ok=True)

app = FastAPI()
client = get_database()

user_id = "1e5cd229-d1cc-4a9a-8338-d0654ca4b575"

@app.post("/my-files/")
async def create_file(file: Annotated[bytes, File()]):
    return {"file_size": len(file)}


@app.post("/upload-file/")
async def create_upload_file(file: UploadFile | None = None):
    if file:
        create_folder(user_id)
        collection_name = client[user_id]
        collection_name.insert_one({"_id": str(uuid.uuid4()), "filename": file.filename, "filepath": f"files/{user_id}/{file.filename}"})
        with open(f'files/{user_id}/{file.filename}', 'wb') as new_file:
            new_file.write(await file.read())
            return {"message": "uploaded"}
    else:
        return {"message": "No file sent"}
    

@app.get("/")
async def index() -> dict:
    return {"Hello": "World!"}


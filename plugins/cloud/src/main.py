from fastapi import FastAPI
from typing import Dict


# Initialize fastapi application
app = FastAPI()


@app.get('/')
async def index() -> Dict[str, str]:
    return {'Hello': 'World!'}


@app.get('/my-files')
async def list_of_my_files() -> Dict[str, str]:
    return {'Hello': 'World!'}


@app.post('/upload-file')
async def upload_file() -> Dict[str, str]:
    return {'Hello': 'World'}


@app.post('/create-directory')
async def create_directory() -> Dict[str, str]:
    return {'Hello': 'World!'}


@app.get('/get-info')
async def get_node_info() -> Dict[str, str]:
    return {'Hello': 'World!'}


@app.get('/download-file')
async def download_file() -> Dict[str, str]:
    return {'Hello': 'World!'}


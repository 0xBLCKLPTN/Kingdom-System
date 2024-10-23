import uuid
from typing import List

class Bucket:
    def __init__(self: object, author_id: str) -> None:
        self.author_id: str = author_id
        self.name = uuid.uuid4()
        self.files: List = []

    
    def __str__(self) -> str:
        return f"AUTHOR_ID: {self.author_id}\nBUCKET NAME: {self.name}"
    

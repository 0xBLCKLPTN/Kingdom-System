import json

def log(color: str, status: str, message: str) -> None:
    pass

class ParseSettings:
    def __init__(self, conf: str):
        self.conf: str = conf

    def read_settings(self):
        pass

    def download_repo(self, url: str) -> bool:
        pass



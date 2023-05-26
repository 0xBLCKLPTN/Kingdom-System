from pydantic import BaseSettings

class Settings(BaseSettings):
    BOT_TOKEN: str

settings = Settings()

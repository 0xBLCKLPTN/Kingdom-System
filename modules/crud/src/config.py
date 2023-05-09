from pydantic import BaseSettings

class Settings(BaseSettings):
    AMQPHOST: str
    REDISHOST: str
    REDISPORT: int

settings = Settings()
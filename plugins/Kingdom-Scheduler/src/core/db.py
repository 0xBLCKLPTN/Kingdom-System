from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

ENGINE='postgresql'
USERNAME='super_postgres'
PASSWORD='super_postgres'
HOST='127.0.0.1:5432'
DB_NAME='scheduler_dev'

SQLALCHEMY_DATABASE_URL = f'{ENGINE}://{USERNAME}:{PASSWORD}@{HOST}/{DB_NAME}'

engine = create_engine(SQLALCHEMY_DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

def create_database():
    Base.metadata.create_all(bind=engine)
    return SessionLocal()

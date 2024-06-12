from sqlalchemy import Column, DateTime, Integer, String, BigInteger
from sqlalchemy.dialects.postgresql import UUID
import uuid
from core.db import Base

class UsersDBModel(Base):
    __tablename__ = 'users'

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    firstName = Column(String, nullable=False)
    middleName = Column(String, nullable=True)
    lastName = Column(String, nullable=False)

    role = Column(String, nullable=False, default='student')
    hashed_password = Column(String, nullable=False)
    username = Column(String, nullable=False)


class GroupsDBModel(Base):
    __tablename__ = 'groups'

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String, nullable=False)
    course = Column(Integer, nullable=True)
    budget = Column(Integer, default=1)


class LessonsDBModel(Base):
    __tablename__ = 'lessons'

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String, nullable=False)

from Controllers.documentsController import DocumentsController
from Controllers.mongoController import MongoController
from Controllers.postgresController import PostgresController
from Controllers.redisController import RedisController


# Base crud for all databases and values.
class Crud(DocumentsController, MongoController, PostgresController, RedisController):
    def __init__(self):
        ...

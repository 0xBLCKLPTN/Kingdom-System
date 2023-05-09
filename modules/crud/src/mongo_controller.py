from pymongo import MongoClient

class MongoController:
    def __init__(self):
        self.client = MongoClient("localhost", 27017)
    
from pymongo import MongoClient

def get_database():
    client = MongoClient("localhost", 27017)
    return client['boxed_files']
    

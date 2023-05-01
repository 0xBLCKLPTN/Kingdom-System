import pymongo


class MongoController:
    def __init__(self):
        self.client = pymongo.MongoClient("localhost", 27017)
        self.db = self.client['SeriesDB']
        self.collection_name = self.db['series']
    
    def insertMany(self, data):
        self.collection_name.insert_many(data)
    
    def get_data(self):
        item_details = self.collection_name.find()
        for item in item_details:
            # This does not give a very readable output
            print(item)

item_1 = {
  "_id" : "U1IT00001",
  "item_name" : "Blender",
  "max_discount" : "10%",
  "batch_number" : "RR450020FRG",
  "price" : 340,
  "category" : "kitchen appliance"
}

item_2 = {
  "_id" : "U1IT00002",
  "item_name" : "Egg",
  "category" : "food",
  "quantity" : 12,
  "price" : 36,
  "item_description" : "brown country eggs"
}

MongoController().get_data()
#.insertMany([item_1, item_2])

from pymongo import MongoClient

class GetRequests:
    def get_schedule_by(self, by: str = 'timestamp', value: str = 'none') -> object:
        return self.client.schedules.find_one({ by : value })

    def get_schedules_by(self, by: str = 'timestamp', value: str = 'none') -> object:
        return self.client.schedules.find_many({ by : value })


class RemoveRequests:
    def remove_schedule_by(self, by: str = 'timestamp', value: str = 'none') -> None:
        self.client.schedules.delete_one({ by : value })

    def remove_schedules_by(self, by: str = 'timestamp', value: str = 'none') -> None:
        self.client.schedules.delete_many({ by : value })


class AddRequests:
    def add_schedule(self, by: str = 'timestamp', value: str ='none') -> None:
        self.client.schedules.insert_one({ by : value })


class MongoController(GetRequests, RemoveRequests, AddRequests):
    def __init__(self):
        client = MongoClient('localhost', 27017)
        self.client = client.kingdom
   
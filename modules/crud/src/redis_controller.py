import redis

class RedisController:
	def __init__(self):
		self.r = redis.Redis(host='localhost', port=6379, decode_responses=True)
		print("Connection Successful initialized!")
		
	def add_data(self, key, value):
        try:
		    self.r.set(key, value)
		    print("Message sended!")
        except Exception as ex:
            print(ex)

RedisController().add_data()

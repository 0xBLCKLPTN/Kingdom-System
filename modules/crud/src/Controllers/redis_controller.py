import redis
#from logger import test_logger

class RedisController:
	def __init__(self):
		self.r = redis.Redis(host='localhost', port=6379, decode_responses=True)
		#test_logger.info("[RedisController]: has been started!")

	def add_data(self, key, value):
		self.r.set(key, value)
	
	def get_value(self, key):
		return self.r.get(key)

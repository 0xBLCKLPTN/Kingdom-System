from amqp_controller import AMQPController
from redis_controller import RedisController

class Controller(AMQPController, RedisController, MongoController):
    def __init__(self, *args, **kwargs):
        super().__init__()
    
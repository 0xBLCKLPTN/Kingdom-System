import pika
#from logger import test_logger

def callback(ch, method, properties, body):
    if body == b"api_need_teachers":
        print("I need a teachers!")

    if body == b"updates_is_ready_ask":
        print("API: i need updates!")


class AMQPController:
    def __init__(self):
        try:
            connection: object = pika.BlockingConnection(pika.ConnectionParameters(host='127.0.0.1'))
            channel: object = connection.channel()
            channel.queue_declare(queue="crud")
            channel.basic_consume(queue='crud', on_message_callback=callback, auto_ack=True)
            #test_logger.info("[AMQPController]: has been initilized and wait messages!")
            print("Initialozed")
            channel.start_consuming()


        except Exception as ex:
            #test_logger.exception(f"[AMQPController]: has been crashed while connecting. Full error: {ex}")
            print(ex)
AMQPController()


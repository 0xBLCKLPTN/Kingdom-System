import pika

def callback(ch, method, properties, body):
    print(" [x] Received %r" % body)


class AMQPController:
    def __init__(self):
        connection: object = pika.BlockingConnection(pika.ConnectionParameters(host='127.0.0.1'))
        channel: object = connection.channel()
        channel.queue_declare(queue="crud")
        channel.basic_consume(queue='crud', on_message_callback=callback, auto_ack=True)
        print(' [*] Waiting for messages. To exit press CTRL+C')
        channel.start_consuming()    

AMQPController()

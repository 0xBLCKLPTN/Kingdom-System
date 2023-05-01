import pika

class AMQPreceiver:
    def __init__(self) -> None:
        self.connection = pika.BlockingConnection(pika.ConnectionParameters('localhost'))
        self.channel = self.connection.channel()

    def start_receiver(self, queque: str = 'test') -> None:
        self.channel.queue_declare(queue=queque)

        def callback(ch, method, properties, body):
            print(" [x] Received %r" % body)

        self.channel.basic_consume(queue='hello', on_message_callback=callback, auto_ack=True)

        print(' [*] Waiting for messages. To exit press CTRL+C')
        self.channel.start_consuming()


AMQPreceiver().start_receiver()

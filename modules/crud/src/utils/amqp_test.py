import pika

connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

channel.queue_declare(queue='hello')


# Routing key is a queue
channel.basic_publish(exchange='', routing_key='crud', body='need_teachers')
print(" [x] Sent Need Teachers'")
connection.close()

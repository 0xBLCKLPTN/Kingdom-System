use amiquip::{Connection, Channel, Exchange, Publish, Result};

pub struct AMQPConnection {
    connection: Connection,
    channel: Channel,
}

impl AMQPConnection {
    
    // Initialize our connection to rabbit mq.
    pub fn new() -> Self {
        let mut connection: Connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();
        let channel: Channel = connection.open_channel(None).unwrap();

        AMQPConnection { connection, channel }
    }
    
    // Send data to Rabbit MQ via amqp protocol.
    pub async fn publish_data(&self, queque_name: &str, data: &str) {
        let exchange = Exchange::direct(&self.channel);
        exchange.publish(Publish::new(data.as_bytes(), quque_name)).unwrap();
    }

    pub async fn close_connection(&self) {
        self.connection.close();
    }
}

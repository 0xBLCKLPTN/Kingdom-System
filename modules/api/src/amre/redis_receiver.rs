use redis::{Connection, Client, Commands};
use redis;

// Redis connection pool with client for KS-api.
pub struct Database {
    client: Client,
    connection: Connection,
}


impl Database {
    
    // Create a new connection pool > struct for other functions.
    // OOP-programming implementation
    pub fn new () -> Self {
        let client: Client = Client::open("redis://127.0.0.1:6379/").unwrap();
        let mut connection: Connection = client.get_connection().unwrap();

        Database {
            client,
            connection,
        }
    }
    
    // Get some data via key from redis DB.
    pub fn get_data(&mut self, key: &str) -> String {
        let data: String = redis::cmd("GET")
            .arg(key)
            .query(&mut self.connection)
            .expect("failed to execute GET for some key");
        data
    }
}

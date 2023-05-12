extern crate r2d2_redis;

use r2d2_redis::{RedisConnectionManager};
//use r2d2_redis::redis::{Commands};
//use r2d2_redis::redis::RedisError;

extern crate redis;
use redis::{Client, JsonCommands, Connection, ConnectionLike, RedisError};
use r2d2;

type Pool = r2d2::Pool<Client>;

#[derive(Clone)]
pub struct RedisGetter {
    pool: Pool,
}

impl RedisGetter {
    pub fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let pool = r2d2::Pool::builder()
            .build(client)
            .unwrap();
        Self {pool}
    }
    

    
    pub fn get_data(&self, key: &str) -> String {
        let mut con = self.pool.get().unwrap();
        
        // Json_get currently not work, because it cant find a path? what is path?
        let result: Result<String, RedisError> = con.json_get(key, "$");
        match result {
            Ok(_) => return result.unwrap(),
            Err(_) => return "Cannot find teacher".to_string(),
        }        
    }
}



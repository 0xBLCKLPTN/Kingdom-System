extern crate r2d2_redis;

use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

type Pool = r2d2::Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisGetter {
    pool: Pool,
}

impl RedisGetter {
    pub fn new() -> Self {
        let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
        let pool = r2d2::Pool::builder()
            .build(manager)
            .unwrap();
        Self { pool }
    }

    pub fn get_data(&self, key: &str){
        let mut con = self.pool.get().unwrap();
        let teacher: String = con.get(key).unwrap();
        println!("{}", teacher);
    }
}



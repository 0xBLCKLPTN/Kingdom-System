pub mod atypes;
pub mod configurator;
pub mod fs_manager;

pub mod remore_db;
pub mod log_db;
pub mod queue_db;


use fs_manager::*;
use atypes::*;
use configurator::*;
use std::env;

use std::path::PathBuf;

use log_db::*;
use remore_db::*;
use queue_db::*;

struct AliceDatabase {
    pub log_db_mod: LogDatabase,
    pub remore_db_mod: RemoreDBInstance,
    pub queue_db_mod: Queue,
}

impl AliceDatabase {
    pub fn new(init_all: bool) -> Self {
        let mut log_db_mod = LogDatabase::new(PathBuf::from("./LOGDATABASE"));
        let mut remore_db_mod = RemoreDBInstance::new("name".to_string(), Some("127.0.0.1".to_string()), Some(12_u8), "SUPERKEY".to_string());
        let mut queue_db_mod =Queue::new();
        AliceDatabase { log_db_mod, remore_db_mod, queue_db_mod}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut adb = AliceDatabase::new(true);
    Ok(())
    
}

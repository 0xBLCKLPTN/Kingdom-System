pub mod atypes;
pub mod configurator;
pub mod fs_manager;

pub mod remore_db;
pub mod log_db;
pub mod queue_db;
pub mod default_db;
pub mod adbcore;

use fs_manager::*;
//use atypes::*;
use configurator::*;
use std::env;
//use adbcore;
use std::path::PathBuf;
use crate::adbcore::DatabaseController;
use crate::adbcore::Databases;
/*
use log_db::*;
use remore_db::*;
use queue_db::*;
use default_db::*;
use adbcore;

struct AliceDatabase {
    pub log_db_mod: Option<LogDatabase>,
    pub remore_db_mod: Option<RemoreDBInstance>,
    pub queue_db_mod: Option<Queue>,
}

impl AliceDatabase {
    pub fn new(init_all: bool) -> Self {
        let mut log_db_mod = Some(LogDatabase::new(PathBuf::from("./LOGDATABASE")));
        let mut remore_db_mod = Some(RemoreDBInstance::new("name".to_string(), Some("127.0.0.1".to_string()), Some(12_u8), "SUPERKEY".to_string()));
        let mut queue_db_mod = Some(Queue::new());
        AliceDatabase { log_db_mod, remore_db_mod, queue_db_mod}
    }
}
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();    
    let conf = read_config(&args[1]).unwrap();

    let mut dbs = Databases::new();

    for db in conf {
        let mut k = adbcore::Database::new(PathBuf::from(db.database_path), db.database_type.as_str());
        for table in db.tables {
            k.create_table(table.name.as_str()).await;
        }
        dbs.add(k);
    }

    let k = dbs.get_db_by_name("DATABASE".to_string());
    match k {
        Some(db) => println!("{:#?}", db.get_table_by_name("ananas".to_string()).await),
        _ => println!("I Cant find db"),
    }
    Ok(())
    
}

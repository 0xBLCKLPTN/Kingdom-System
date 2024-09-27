pub mod atypes;
pub mod configurator;
pub mod remore_db;
pub mod fs_manager;
pub mod log_db;

use log_db::*;
use fs_manager::*;
use atypes::*;
use configurator::*;
use std::env;
use remore_db::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let mut ld = LogDatabase::new(PathBuf::from("./LOGDATABASE"));
    ld.find_tables();
    ld.create_table("logs").await;
    println!("{:#?}", ld);
}


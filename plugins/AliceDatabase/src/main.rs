pub mod atypes;
pub mod configurator;
pub mod remore_db;
pub mod fs_manager;
pub mod log_db;
pub mod queue_db;

use log_db::*;
use fs_manager::*;
use atypes::*;
use configurator::*;
use std::env;
use remore_db::*;
use std::path::PathBuf;
use queue_db::{Queue, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let queue = Queue::new();
    queue.enqueue(Command {command: "new_table: qwe".to_string(), author: "Daniil Ermolaev".to_string() });
    queue.enqueue(Command {command: "new_table: qwe2".to_string(), author: "Daniil Ermolaev".to_string() });
    Ok(())
    
}

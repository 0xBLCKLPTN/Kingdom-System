//!
//!    Filename: main.rs
//!    Description: Start point of AliceDB plugin.
//!    Author: Daniil (0xJanus) Ermolaev
//!

pub mod adbcore;
use adbcore::types::BoxedResult;

use std::path::PathBuf;
use adbcore::fs_manager::*;
use adbcore::executor::*;

use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "sql.pest"]
struct IdentParser;


#[tokio::main]
async fn main() -> BoxedResult<()> {
    execute_command("CREATE TABLE testtable; CREATE DATABASE qwe;qweqwe").await;
    Ok(())
}

pub mod tests;
use tests::*;

//!
//!    Filename: main.rs
//!    Description: Start point of AliceDB plugin.
//!    Author: Daniil (0xJanus) Ermolaev
//!
#[macro_use]
extern crate lazy_static;

pub mod adbcore;
pub mod engines;
use adbcore::types::BoxedResult;
use adbcore::executor::execute_command;

use engines::default_adbp::*;

fn print_ascii() {
    println!(r"
    @---------------------------------------------------------------@
    |        ______     __         __     ______     ______         | 
    |       /\  __ \   /\ \       /\ \   /\  ___\   /\  ___\        |
    |       \ \  __ \  \ \ \____  \ \ \  \ \ \____  \ \  __\        |
    |        \ \_\ \_\  \ \_____\  \ \_\  \ \_____\  \ \_____\      |
    |         \/_/\/_/   \/_____/   \/_/   \/_____/   \/_____/      |
    |                                                               |
    |                     _____     ______                          |
    |                    /\  __-.  /\  == \                         |
    |                    \ \ \/\ \ \ \  __<                         |
    |                     \ \____-  \ \_____\                       |
    |                      \/____/   \/_____/                       |
    |                                                               |
    @---------------------------------------------------------------@
    ",);
}



#[tokio::main]
async fn main() -> BoxedResult<()> {
    print_ascii();
    ///println!("{:#?}", get_databases().await);
    //println!("{:#?}", get_tables_in_databases().await);
    //execute_command("CREATE DATABASE database1 ENGINE default;").await;
    //execute_command("CREATE DATABASE database2 ENGINE default;").await;
    //execute_command("CREATE TABLE default3.testtable;").await;
    //execute_command("PRINTALLDB").await;
    //execute_command("INSERT (data, qwe) INTO default3.testtable;").await;
    Ok(())
}
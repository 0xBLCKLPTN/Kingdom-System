//!
//!    Filename: main.rs
//!    Description: Start point of AliceDB plugin.
//!    Author: Daniil (0xJanus) Ermolaev
//!
#[macro_use]
extern crate lazy_static;

use termion::color;

macro_rules! adbprint {
    ($($arg:tt)*) => {
        println!("[ {}ADB{} ]=: {}", color::Fg(color::Yellow), color::Fg(color::Reset), format!($($arg)*));
    };
}

pub mod adbcore;
pub mod engines;
pub mod configurator;


use adbcore::types::BoxedResult;
use adbcore::executor::execute_command;
use engines::default_adbp::*;
use configurator::read_config;
use adbcore::pickling::*;
use std::process::Command;

fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear the screen");
    }
}

fn print_ascii() {
    clear();
    println!(r"
    @---------------------------------------------------------------@
    |       {} ______     __         __     ______     ______{}         | 
    |       {}/\  __ \   /\ \       /\ \   /\  ___\   /\  ___\{}        |
    |       {}\ \  __ \  \ \ \____  \ \ \  \ \ \____  \ \  __\{}        |
    |        {}\ \_\ \_\  \ \_____\  \ \_\  \ \_____\  \ \_____\{}      |
    |         {}\/_/\/_/   \/_____/   \/_/   \/_____/   \/_____/{}      |
    |                                                               |
    |                    {} _____     ______{}                          |
    |                    {}/\  __-.  /\  == \{}                         |
    |                    {}\ \ \/\ \ \ \  __<{}                         |
    |                     {}\ \____-  \ \_____\{}                       |
    |                      {}\/____/   \/_____/{}                       |
    |                                                               |
    @---------------------------------------------------------------@
    ",
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),
    color::Fg(color::Yellow), color::Fg(color::Reset),

);
}

async fn prepair_using_config(config_path: &str) {
    let config = read_config(config_path).unwrap();
    for database in config {
        execute_command(&("CREATE DATABASE ".to_owned() + &database.database_name + " ENGINE " + &database.database_type +";")).await;
        for table in database.tables {
            execute_command(&("CREATE TABLE ".to_owned() + &database.database_name + "." + &table.name +";")).await;
            adbprint!("{:#?}", table.name);
        }
        
    }
}


#[tokio::main]
async fn main() -> BoxedResult<()> {
    print_ascii();
    prepair_using_config("./config.json").await;
    //execute_command("SHOW DATABASES;").await;
    execute_command("INSERT (data1, data2) INTO auth.users;").await;
    //execute_command("CREATE DATABASE " + config.database_name + ";").await;
    ///println!("{:#?}", get_databases().await);
    //println!("{:#?}", get_tables_in_databases().await);
    //execute_command("CREATE DATABASE database1 ENGINE default;").await;
    //execute_command("CREATE DATABASE database2 ENGINE default;").await;
    //execute_command("CREATE TABLE default3.testtable;").await;
    //execute_command("PRINTALLDB").await;
    //execute_command("INSERT (data, qwe) INTO default3.testtable;").await;
    Ok(())
}
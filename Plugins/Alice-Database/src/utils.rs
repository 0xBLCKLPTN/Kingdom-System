/*                          MIT License

Copyright (c) 2024 Daniil Ermolaev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */

use std::{ fs, env };
use std::io::{ self, Read, Write };
use std::path::{ PathBuf, Path };

use log::{ info, error, trace };
use simplelog::*;

use chrono::Local;

const ROOT_DIR: &str = "Alice-Database-Data";
const ADB_DATA_DIR: &str = "ADB_Data";
const JSON_ENGINE_DIR: &str = "json_engine";
const ADB_LOGS_DIR: &str = "ADB_Logs";

pub fn get_root_path() -> PathBuf {
    let root_path = match prepare() {
        Ok(k) => k,
        _ => panic!("Errors while preparing..."),
    };
    return root_path;
}


pub fn prepare() -> std::io::Result<PathBuf> {
    print_ascii();
    // Get the home directory
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let base_path = home_dir.join(ROOT_DIR);
    let adb_data_path = base_path.join(ADB_DATA_DIR);
    let adb_logs_path = base_path.join(ADB_LOGS_DIR);

    // Ensure the base and log directories exist
    fs::create_dir_all(&adb_data_path).expect("Failed to create ADB_Data directory");
    fs::create_dir_all(&adb_logs_path).expect("Failed to create ADB_Logs directory");

    // Define the data path for JSON documents
    let root_path = adb_data_path.join(JSON_ENGINE_DIR);

    // Ensure the JSON engine directory exists
    fs::create_dir_all(&root_path).expect("Failed to create json_engine directory");
    // Generate a unique log filename using timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_path = format!("{}/{}.adb.log", adb_logs_path.display(), timestamp);

    // Set up logging configuration
    let log_config = ConfigBuilder::new().build();

    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Trace,
                log_config.clone(),
                TerminalMode::Mixed,
                ColorChoice::Auto
            ),
            WriteLogger::new(
                LevelFilter::Trace,
                log_config.clone(),
                fs::File::create(log_file_path)
                    .unwrap()
            ),
        ]
    ).unwrap();
    Ok(root_path.clone())
    
}

pub fn print_ascii() {
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
    ")
}

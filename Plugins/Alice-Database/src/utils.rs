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

pub fn prepare() -> std::io::Result<PathBuf> {
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

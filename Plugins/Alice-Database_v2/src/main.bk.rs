//! # ADB Instance Manager
//!
//! This module implements functionalities for managing ADB instances with different engine types,
//! providing capabilities for logging and JSON data management. The module includes the following
//! main components: `LogEngine`, `JSONEngine`, `ADBInstance`, and `ADBInstanceManager`.
//!
//! ## Components
//!
//! - `LogEngine`: A struct for handling logs with file-based storage.
//! - `JSONEngine`: A struct for managing JSON data.
//! - `ADBInstance`: Represents an instance of a database engine, which can be either
//!   `LogEngine` or `JSONEngine`.
//! - `ADBInstanceManager`: Manages multiple `ADBInstance`s, enabling creation, retrieval, and
//!   manipulation of instances.

#[macro_use]
extern crate pest_derive;

use uuid::Uuid;
use tokio::fs::{self, File as AsyncFile};
use tokio::io::AsyncWriteExt;
use log::{info, error};
use colored::*;

/// Prints an ASCII art header.
fn print_ascii() {
    println!("{}", r#"
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

    "#.green());
}

/// A structure that represents a log engine.
#[derive(Debug, Clone)]
pub struct LogEngine {
    /// Unique name of the log engine.
    pub name: String,
    /// Stored log entries.
    pub logs: Vec<String>,
}

/// A structure that represents a JSON engine.
#[derive(Debug, Clone)]
pub struct JSONEngine {
    /// Unique name of the JSON engine.
    pub name: String,
    /// Stored JSON data.
    pub data: Vec<String>,
}

/// A structure that represents an ADB instance.
#[derive(Debug, Clone)]
pub struct ADBInstance {
    /// Unique name of the instance.
    pub name: String,
    /// The engine type used by the instance.
    pub engine: EngineType,
}

/// A structure that manages multiple ADB instances.
#[derive(Debug, Clone)]
pub struct ADBInstanceManager {
    /// Unique name of the instance manager.
    pub name: String,
    /// List of managed instances.
    pub instances: Vec<ADBInstance>,
}

/// Enum that represents the type of engine used in an ADB instance.
#[derive(Debug, Clone)]
pub enum EngineType {
    /// Log engine variant.
    Log(LogEngine),
    /// JSON engine variant.
    JSON(JSONEngine),
}

impl LogEngine {
    /// Creates a new `LogEngine` instance with a unique name.
    pub fn new() -> Self {
        Self { name: Uuid::new_v4().to_string(), logs: Vec::new() }
    }

    /// Asynchronously writes log data to a log file associated with the instance.
    pub async fn write(&mut self, instance_name: &str, data: &str) {
        let filepath = format!("ADB_Data/log_engine/{}/{}.alicedb.log", instance_name, self.name);
        match AsyncFile::options().append(true).create(true).open(&filepath).await {
            Ok(mut file) => {
                if let Err(e) = file.write_all((data.to_owned() + "\n").as_bytes()).await {
                    error!("Failed to write data to {}: {}", filepath, e);
                } else {
                    info!("Data written to {}", filepath);
                }
            }
            Err(e) => {
                error!("Failed to open file {}: {}", filepath, e);
            }
        }
    }
}

impl JSONEngine {
    /// Creates a new `JSONEngine` instance with a unique name and logs the creation.
    pub fn new() -> Self {
        info!("Creating JSON Engine....");
        Self { name: Uuid::new_v4().to_string(), data: Vec::new() }
    }
}

impl ADBInstanceManager {
    /// Creates a new `ADBInstanceManager`, setting up the necessary directory structure.
    pub async fn new() -> Self {
        fs::create_dir_all("ADB_Data").await.expect("Failed to create data directory");
        info!("Instance Manager created!");
        Self { name: Uuid::new_v4().to_string(), instances: Vec::new() }
    }

    /// Finds all existing ADB instances and returns the manager with those instances.
    pub async fn find_all() -> tokio::io::Result<Self> {
        let mut instances = Vec::new();
        let paths = std::fs::read_dir("ADB_Data/")?;

        for entry in paths.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let engine_type = path.file_name().unwrap().to_string_lossy().to_string();
                let instance_paths = std::fs::read_dir(path)?;

                for instance_entry in instance_paths.filter_map(Result::ok) {
                    let instance_name = instance_entry.file_name().to_string_lossy().to_string();
                    instances.push(ADBInstance::new_from_exist(instance_name, engine_type.clone()));
                }
            }
        }
        info!("Instance Manager found all dbs!");
        Ok(Self { name: Uuid::new_v4().to_string(), instances })
    }

    /// Creates a new ADB instance with the specified engine name.
    pub async fn create_new_instance(&mut self, engine_name: &str) -> String {
        let instance = ADBInstance::new(engine_name);
        self.instances.push(instance.clone());

        if fs::create_dir_all(format!("ADB_Data/{}/{}", engine_name, instance.name)).await.is_err() {
            error!("Failed to create instance directory");
            return String::new();
        }

        instance.name
    }

    /// Retrieves a mutable reference to an ADB instance by its name, if it exists.
    pub fn get_instance(&mut self, instance_name: &str) -> Option<&mut ADBInstance> {
        self.instances.iter_mut().find(|instance| instance.name == instance_name)
    }
}

impl ADBInstance {
    /// Creates a new `ADBInstance` with the specified engine name.
    pub fn new(engine_name: &str) -> Self {
        info!("Creating new ADBInstance...");
        let engine = match engine_name {
            "log_engine" => EngineType::Log(LogEngine::new()),
            "json_engine" => EngineType::JSON(JSONEngine::new()),
            _ => {
                error!("No engine selected!");
                panic!("No engine selected!");
            },
        };
        info!("ADBInstance created!");
        Self { name: Uuid::new_v4().to_string(), engine }
    }

    /// Creates a new `ADBInstance` from existing information.
    pub fn new_from_exist(name: String, engine_type: String) -> Self {
        info!("Creating new ADBInstance from exist...");
        let engine = match engine_type.as_str() {
            "log_engine" => EngineType::Log(LogEngine::new()),
            "json_engine" => EngineType::JSON(JSONEngine::new()),
            _ => {
                error!("No engine selected!");
                panic!("No engine selected!");
            },
        };
        info!("ADBInstance created!");
        Self { name, engine }
    }
}

/// Main entry point of the application.
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info"); // Set logging level.
    env_logger::init(); // Initialize logger.
    print_ascii(); // Print ASCII art.

    let mut instance_manager = match ADBInstanceManager::find_all().await {
        Ok(manager) => manager,
        Err(e) => {
            error!("Failed to find instances: {}", e);
            info!("Creating new instance manager...");
            ADBInstanceManager::new().await
        },
    };

    // Example of using the instance manager
    let id = instance_manager.instances[0].name.clone();
    if let Some(instance_engine) = instance_manager.get_instance(&id) {
        if let EngineType::Log(log_engine) = &mut instance_engine.engine {
            // Write log messages to the log engine.
            log_engine.write(&id, "message").await;
            log_engine.write(&id, "message2").await;
        } else {
            println!("No method found!");
        }
    }

    // Example of creating new instances
    // let mut instance_manager = ADBInstanceManager::new().await.unwrap();
    // let id = instance_manager.create_new_instance("log_engine").await;
    // let id2 = instance_manager.create_new_instance("json_engine").await;
}

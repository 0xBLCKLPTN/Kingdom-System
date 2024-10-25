use std::{ env, fs };
use std::io::{ self, Read, Write };
use std::path::{ PathBuf, Path };
use std::sync::{ Arc, Mutex };

use serde_json::{ json, Value, Result as JsonResult };

use log::{ info, error, trace };
use simplelog::*;

use chrono::Local;

pub mod json_engine;
pub mod engines;
pub mod grpc_server;
pub mod instance;
pub mod utils;

use json_engine::*;
use engines::*;
use grpc_server::*;
use instance::*;
use utils::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_ascii();
    
    let root_path = match prepare() {
        Ok(k) => k,
        _ => panic!("Errors in prepare function."),
    };

    let instance_manager = GRPCInstanceManager {
        instance_manager: Arc::new(Mutex::new(InstanceManager::new(&root_path))),
    };
    
    println!("Starting gRPC server...");

    Server::builder()
        .add_service(InstanceServiceServer::new(instance_manager))
        .serve("[::1]:50052".parse()?)
        .await?;

    Ok(())
}

use std::fs;
use std::io::{self, Read, Write};
use std::path::{PathBuf, Path};
use serde_json::{json, Value, Result as JsonResult};
use log::{info, error, trace};
use simplelog::*;
use chrono::Local;
use std::env; // Import this for getting the home directory

pub mod json_engine;
use json_engine::*;
pub mod engines;
use engines::Engines;
pub mod instance;
use instance::*;

use crate::instance::InstanceManager;


use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use std::sync::{Arc, Mutex};

pub mod instance_g {
    tonic::include_proto!("instance");
}
pub use instance_g::{
    CreateInstanceRequest, CreateInstanceResponse,
    GetInstanceRequest, GetInstanceResponse,
    GetAllInstancesRequest, GetAllInstancesResponse,
    //SignInRequest, SignInResponse,
    SignUpRequest, SignUpResponse,
    Token,

};

use crate::instance_g::instance_service_server::InstanceServiceServer;
use crate::instance_g::instance_service_server::InstanceService;

// Define constants for the root path and log path
const ROOT_DIR: &str = "Alice-Database-Data";
const ADB_DATA_DIR: &str = "ADB_Data";
const JSON_ENGINE_DIR: &str = "json_engine";
const ADB_LOGS_DIR: &str = "ADB_Logs";

fn prepare() -> std::io::Result<PathBuf> {
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
            TermLogger::new(LevelFilter::Trace, log_config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
                            WriteLogger::new(LevelFilter::Trace, log_config.clone(), fs::File::create(log_file_path).unwrap()),
        ]
    ).unwrap();
    Ok(root_path.clone())
    
}

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
    ")
}

// Define the gRPC service
#[derive(Debug, Default)]
pub struct MyInstanceManager {
    pub instances: Arc<Mutex<InstanceManager>>,
}

#[tonic::async_trait]
impl InstanceService for MyInstanceManager {
    
    async fn create_instance(
        &self,
        request: Request<CreateInstanceRequest>,
    ) -> Result<Response<CreateInstanceResponse>, Status> {
        let inner = request.into_inner();
        let engine_type = inner.engine_type; // assuming root_path is a string path
        let mut manager = self.instances.lock().unwrap();
        let id = manager.create_instance(&engine_type);
        
        let response = CreateInstanceResponse { instance: id };
        Ok(Response::new(response))
    }

    /// Get an existing instance by name

    async fn get_instance(
        &self,
        request: Request<GetInstanceRequest>,
    ) -> Result<Response<GetInstanceResponse>, Status> {
        let instance_name = request.into_inner().instance_name;
        // Lock the mutex to safely access the manager
        let manager = self.instances.lock().unwrap();
        // Retrieve the instance by name
        if let Some(instance) = manager.get_instance(&instance_name) {
            return Ok(Response::new(GetInstanceResponse {
                instance: instance.name.clone(),  // Clone the entire instance
            }));
        }
        Err(Status::not_found("Instance not found"))
    }

    async fn get_all_instances(
        &self,
        request: Request<GetAllInstancesRequest>,
    ) -> Result<Response<GetAllInstancesResponse>, Status> {
        let manager = self.instances.lock().unwrap();
        let mut re_instances: Vec<instance_g::Instance> = vec![];
        for i in &manager.instances {
            re_instances.push(instance_g::Instance {name: i.name.clone(), engine: "unknown".into()} );
        }
        let response = GetAllInstancesResponse { instances: re_instances };
        Ok(Response::new(response))

    }

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let inner = request.into_inner();
        let mut manager = self.instances.lock().unwrap();
        let mut key: String = String::new();
        if !manager.authenticated_apps.contains_key(&inner.app_name) {
            key = manager.sign_up(inner.app_name);
        } else {
            key = "whoops...".to_string();
        }
        let response = SignUpResponse { secret_key: key };
        
        manager.get_all_apps();
        Ok(Response::new(response))
    }
}


// Main function to start the gRPC server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_ascii();
    let root_path = match prepare() {
        Ok(k) => k,
        _ => panic!("Errors in prepare function.")
    };
    let instance_manager = MyInstanceManager {
        instances: Arc::new(Mutex::new(InstanceManager::new(&root_path))),
    };

    println!("Starting gRPC server...");
    Server::builder()
        .add_service(InstanceServiceServer::new(instance_manager))
        .serve("[::1]:50052".parse()?)
        .await?;

    Ok(())
}


/*
fn main() {
    print_ascii();
    let root_path = match prepare() {
        Ok(k) => k,
        _ => panic!("Errors in prepare function."),
    };

    let mut im = InstanceManger::new();
    let instance_name = im.create_instance("json_engine", &root_path);
    //println!("{:#?}", im);
    let mut engine = im.get_mutable_engine(instance_name.as_str());
    println!("{:#?}", im);
    
}

/// The main entry point for the application.
fn main() -> std::io::Result<()> {
    print_ascii();
    
    
    trace!("Starting Collection Manager...");

    let mut manager = JSONEngine::new(&root_path);

    // Create a new collection
    let collection_name = "collection1"; // Example collection name
    if let Some(_) = manager.add_collection(collection_name) {
        trace!("Created collection: {}", collection_name);
    }

    // Create a new document within the created collection
    let document_name = "document5.json"; // Example document name
    let document_content = json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null
        }
    });

    // Convert JSON Value to String
    let document_content_str = serde_json::to_string(&document_content).expect("Failed to convert JSON to string");

    if let Some(collection) = manager.get_collection_mut(collection_name) {
        if let Err(e) = collection.add_document(document_name, &document_content_str) {
            error!("Failed to add document: {}", e);
        } else {
            trace!("Created document: {} in collection: {}", document_name, collection_name);
        }
    }

    // Example of accessing the document
    if let Some(doc) = manager.get_document(collection_name, document_name) {
        println!("Found document: {:?}", doc);
        trace!("Successfully found document: {}", doc.name);
    } else {
        println!("Document '{}' not found in collection '{}'", document_name, collection_name);
        error!("Document '{}' not found in collection '{}'", document_name, collection_name);
    }

    // Example of updating 'homepage' in the 'payload' field
    if let Some(doc) = manager.get_document_mut(collection_name, document_name) {
        // New value for 'payload.homepage'
        let new_homepage_value = json!("https://new-homepage-url.com");

        if let Err(e) = doc.update_nested_field("payload", "homepage", &new_homepage_value) {
            error!("Failed to update 'homepage' in 'payload': {}", e);
        } else {
            trace!("Updated 'homepage' in document: {}", document_name);
        }
    }

    Ok(())
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use serde_json::json;
    const TEST_ROOT_DIR: &str = "Alice-Database-Data-Test";
    const TEST_ADB_DATA_DIR: &str = "ADB_Data";
    const TEST_JSON_ENGINE_DIR: &str = "json_engine";

    // Setup a temporary test directory
    fn setup_test_dir() -> PathBuf {
        let home_dir = env::temp_dir();
        let test_dir = home_dir.join(TEST_ROOT_DIR);
        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        test_dir
    }

    #[test]
    fn test_create_and_add_document() {
        let root_path = setup_test_dir();
        let mut manager = JSONEngine::new(&root_path);

        // Create a collection
        manager.add_collection("test_collection").unwrap();
        let collection = manager.get_collection_mut("test_collection").unwrap();

        // Add a document
        let doc_content = json!({"code": 200, "success": true}).to_string();
        let result = collection.add_document("test_document.json", &doc_content);
        assert!(result.is_ok());

        // Verify the document is added
        assert_eq!(collection.documents.len(), 1);
        assert_eq!(collection.documents[0].name, "test_document.json");
    }

    #[test]
    fn test_update_document() {
        let root_path = setup_test_dir();
        let mut manager = JSONEngine::new(&root_path);

        // Create a collection and add a document
        manager.add_collection("test_collection").unwrap();
        let collection = manager.get_collection_mut("test_collection").unwrap();
        let doc_content = json!({
            "code": 200,
            "success": true,
            "payload": {
                "homepage": null
            }
        }).to_string();
        collection.add_document("test_document.json", &doc_content).unwrap();

        // Update the homepage field
        let doc = collection.get_document_mut("test_document.json").unwrap();
        let new_homepage_value = json!("https://new-homepage-url.com");

        let update_result = doc.update_nested_field("payload", "homepage", &new_homepage_value);
        assert!(update_result.is_ok());

        // Check the updated value
        assert_eq!(doc.json_value.as_ref().unwrap()["payload"]["homepage"], new_homepage_value);
    }

    #[test]
    fn test_delete_document() {
        let root_path = setup_test_dir();
        let mut manager = JSONEngine::new(&root_path);

        // Create a collection and add a document
        manager.add_collection("test_collection").unwrap();
        let collection = manager.get_collection_mut("test_collection").unwrap();
        collection.add_document("test_document.json", "{\"key\":\"value\"}").unwrap();

        // Ensure the document exists before deletion
        assert_eq!(collection.documents.len(), 1);

        // Delete the document
        let delete_result = collection.delete_document("test_document.json");
        assert!(delete_result.is_ok());

        // Verify the document was deleted
        assert_eq!(collection.documents.len(), 0);
    }

}

use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// Import generated gRPC code
pub mod instance {
    tonic::include_proto!("instance");
}

use instance::{InstanceService, CreateInstanceRequest, CreateInstanceResponse, 
                GetInstanceRequest, GetInstanceResponse, Instance};

// Engines enum represents the types of engines
#[derive(Debug, Clone)]
pub enum Engines {
    JSONEngine(JSONEngine),
}

// Your JSONEngine structure
#[derive(Debug, Clone)]
pub struct JSONEngine {
    // Add fields necessary for your JSON engine
}

impl JSONEngine {
    pub fn new(_root_path: &PathBuf) -> Self {
        // Initialize your JSONEngine as needed
        Self {}
    }
}

// Instance structure
#[derive(Debug, Clone, Default)]
pub struct Instance {
    pub name: String,
    pub engine: Engines,
}

// InstanceManager struct to manage instances
#[derive(Debug, Default)]
pub struct InstanceManager {
    pub name: String,
    pub instances: Vec<Instance>,
}

impl InstanceManager {
    pub fn new() -> Self {
        let name = Uuid::new_v4().to_string();
        let instances: Vec<Instance> = vec![];
        Self { name, instances }
    }

    pub fn create_instance(&mut self, engine_type: &str, root_path: &PathBuf) -> String {
        let instance_name: String = Uuid::new_v4().to_string();

        let engine = match engine_type {
            "json_engine" => Engines::JSONEngine(JSONEngine::new(root_path)),
            _ => panic!("Engine not found"),
        };

        let instance = Instance { engine, name: instance_name.clone() };
        self.instances.push(instance);
        instance_name
    }

    pub fn get_instance(&self, instance_name: &str) -> Option<&Instance> {
        self.instances.iter().find(|i| i.name == instance_name)
    }
}


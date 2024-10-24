use crate::Engines;
use uuid::Uuid;
use std::path::PathBuf;
use crate::JSONEngine;
use std::collections::HashMap;
use ring::{rand::{SecureRandom, SystemRandom}, hmac};
use rand::rngs::OsRng;


#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub engine: Engines,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceManager {
    pub name: String,
    pub instances: Vec<Instance>,
    pub root_path: PathBuf,
    pub authenticated_apps: HashMap<String, String>,
}


impl InstanceManager {
    pub fn new(root_path: &PathBuf) -> Self {
        let name = Uuid::new_v4().to_string();

        let mut instances: Vec<Instance> = vec![];
        let mut authenticated_apps: HashMap<String, String> = HashMap::new();
        Self {name, instances, root_path: root_path.to_owned(), authenticated_apps}
    }

    pub fn create_instance(&mut self, engine_type: &str) -> String {
        let instance_name: String = Uuid::new_v4().to_string();

        let mut engine = match engine_type {
            "json_engine" => Engines::JSONEngine(JSONEngine::new(&self.root_path)),
            _ => panic!("Engine not found"),
        };
        let mut instance = Instance {engine, name: instance_name.clone()};
        self.instances.push(instance);
        instance_name
    }
    pub fn get_instance(&self, instance_name: &str) -> Option<&Instance> {
        self.instances.iter().find(|i| i.name == instance_name)
    }
    // Method to mutate the engine of an existing instance

    pub fn get_mutable_engine(&mut self, instance_name: &str) -> Option<&mut JSONEngine> {
        for instance in &mut self.instances {
            if instance.name == instance_name {
                if let Engines::JSONEngine(ref mut engine) = instance.engine {
                    return Some(engine);
                }
            }
        }
        None
    }

    pub fn sign_up(&mut self, app_name: String) -> String {
        let key = Uuid::new_v4().to_string();
        &self.authenticated_apps.insert(app_name, key.clone());
        return key;
    }

    pub fn get_all_apps(&self) {
        println!("{:#?}", self.authenticated_apps);
    }
}
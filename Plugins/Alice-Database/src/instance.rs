use crate::Engines;
use uuid::Uuid;
use std::path::PathBuf;
use crate::JSONEngine;

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub engine: Engines,
}

#[derive(Debug, Clone)]
pub struct InstanceManger {
    pub name: String,
    pub instances: Vec<Instance>
}


impl InstanceManger {
    pub fn new() -> Self {
        let name = Uuid::new_v4().to_string();
        let mut instances: Vec<Instance> = vec![];
        Self {name, instances}
    }

    pub fn create_instance(&mut self, engine_type: &str, root_path: &PathBuf) -> String {
        let instance_name: String = Uuid::new_v4().to_string();

        let mut engine = match engine_type {
            "json_engine" => Engines::JSONEngine(JSONEngine::new(&root_path)),
            _ => panic!("Engine not found"),
        };
        let mut instance = Instance {engine, name: instance_name.clone()};
        self.instances.push(instance);
        instance_name
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
}
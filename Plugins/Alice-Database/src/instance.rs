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

use crate::Engines;
use uuid::Uuid;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{self, BufRead};
use crate::JSONEngine;
use std::collections::HashMap;
use ring::{rand::{SecureRandom, SystemRandom}, hmac};
use rand::rngs::OsRng;
use crate::IMPestParser;
use pest_derive::Parser;
use pest::Parser;
use crate::Rule;

macro_rules! adbprint {
    ($($arg:tt)*) => {
        println!("[ INSTANCE MANAGER ]=: {}", format!($($arg)*));
    };
}

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

    pub fn create_instance(&mut self, engine_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        let instance_name: String = Uuid::new_v4().to_string();

        let mut engine = match engine_type {
            "json_engine" => Engines::JSONEngine(JSONEngine::new(&self.root_path)),
            _ => {
                println!("Engine not found.");
                return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput, "Engine type not found")));
            }
        };
        let mut instance = Instance {engine, name: instance_name.clone()};
        self.instances.push(instance);
        Ok(instance_name)
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
        adbprint!("{:#?}", self.authenticated_apps);
    }

    pub fn execute_cmd(&mut self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        match IMPestParser::parse(Rule::sql_statements, command) {
            Ok(pairs) => {
                for pair in pairs {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::create_instance => {
                                let inner = inner_pair.into_inner().as_str().split(" ENGINE ").collect::<Vec<_>>();
                                let instance_id = &self.create_instance(&inner[1]);
                                match instance_id {
                                    Ok(message) => adbprint!("NEW INSTANCE ID: {}", message),
                                    Err(e) => adbprint!("{:#?}",e)
                                }
                            },
                            Rule::get_instance => {
                                let inner = inner_pair.into_inner().as_str();
                                adbprint!("{:#?}", inner);
                            },
                            Rule::get_instances => {
                                adbprint!("{:#?}", self.instances);
                            },
                            Rule::print_addbms => {
                                adbprint!("{:#?}", self);
                            }
                            _ => unreachable!("I don't know this command"),
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                adbprint!("Error parsing command: {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn wrapped_execute_cmd(&mut self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        match &self.execute_cmd(command) {
            Ok(_) => println!(""),
            Err(e) => adbprint!("Error! {}", e), 
        }
        Ok(())
    }

    pub fn execute_decl_file<P>(&mut self, filename: P) -> Result<(), io::Error>
    where
        P: AsRef<Path> {
            let file = File::open(filename)?;
            let reader = io::BufReader::new(file);
            let mut lines: Vec<String> = Vec::new();

            for line in reader.lines() {
                self.wrapped_execute_cmd(line?.replace("\n", "").as_str());
            }
            Ok(())
    }
}





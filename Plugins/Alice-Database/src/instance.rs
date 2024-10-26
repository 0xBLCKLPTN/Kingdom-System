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
use crate::{JSONEngine, LOGEngine};
use std::collections::HashMap;
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
        Self {
            name,
            instances: vec![],
            root_path: root_path.to_owned(),
            authenticated_apps: HashMap::new(),
        }
    }

    pub fn create_instance(&mut self, engine_type: &str, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let new_name = name.to_string();
        let engine = match engine_type {
            "json_engine" => Engines::JSONEngine(JSONEngine::new(&self.root_path)),
            "log_engine" => Engines::LOGEngine(LOGEngine::new(&self.root_path)),
            _ => {
                println!("Engine not found.");
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput, "Engine type not found")));
            }
        };
        let instance = Instance { engine, name: new_name.clone() };
        self.instances.push(instance);
        Ok(new_name)
    }

    pub fn get_instance(&self, instance_name: &str) -> Option<&Instance> {
        self.instances.iter().find(|i| i.name == instance_name)
    }

    pub fn get_mutable_engine(&mut self, instance_name: &str) -> Option<&mut Engines> {
        self.instances.iter_mut().find(|instance| instance.name == instance_name)
            .map(|instance| &mut instance.engine)
    }

    pub fn sign_up(&mut self, app_name: String) -> String {
        let key = Uuid::new_v4().to_string();
        self.authenticated_apps.insert(app_name, key.clone());
        key
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
                                let instance_id = self.create_instance(inner[1], inner[0]);
                                match instance_id {
                                    Ok(message) => adbprint!("NEW INSTANCE ID: {}", message),
                                    Err(e) => adbprint!("{:#?}", e),
                                }
                            },
                            Rule::get_instance => {
                                let inner = inner_pair.into_inner().as_str();
                                adbprint!("{:#?}", self.get_instance(inner));
                            },
                            Rule::get_instances => {
                                adbprint!("{:#?}", self.instances);
                            },
                            Rule::print_addbms => {
                                adbprint!("{:#?}", self);
                            },
                            Rule::create_collection => {
                                let inner = inner_pair.into_inner().as_str().split(" INSTANCE WITH NAME ").collect::<Vec<_>>();
                                if let Some(engine) = self.get_mutable_engine(inner[0]) {
                                    match engine {
                                        Engines::JSONEngine(json_engine) => {
                                            json_engine.add_collection(inner[1]); // Call method for JSONEngine
                                        },
                                        Engines::LOGEngine(log_engine) => {
                                            log_engine.add_collection(inner[1]); // Call method for LOGEngine
                                        },
                                        // You can add more engine variants here if needed
                                    }
                                } else {
                                    adbprint!("Instance not found: {}", inner[0]);
                                }
                            }
                            
                            Rule::create_document => {
                                let inner = inner_pair.into_inner().as_str().split(" INSTANCE IN COLLECTION ").collect::<Vec<_>>();
                                let inner_two = inner[1].split(" WITH NAME ").collect::<Vec<_>>();
                            
                                if let Some(engine) = self.get_mutable_engine(inner[0]) {
                                    match engine {
                                        Engines::JSONEngine(json_engine) => {
                                            if let Some(collection) = json_engine.get_collection_mut(inner_two[0]) {
                                                collection.add_document(inner_two[1], "");
                                                adbprint!("DOCUMENT {} CREATED IN {}.{} C.I", inner_two[1], inner_two[0], inner[0]);
                                            } else {
                                                adbprint!("Collection not found in JSONEngine: {}", inner_two[0]);
                                            }
                                        },
                                        Engines::LOGEngine(log_engine) => {
                                            if let Some(collection) = log_engine.get_collection_mut(inner_two[0]) {
                                                collection.add_document(inner_two[1], "");
                                                adbprint!("DOCUMENT {} CREATED IN {}.{} C.I", inner_two[1], inner_two[0], inner[0]);
                                            } else {
                                                adbprint!("Collection not found in LOGEngine: {}", inner_two[0]);
                                            }
                                        },
                                    }
                                } else {
                                    adbprint!("Instance not found: {}", inner[0]);
                                }
                            },                            
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
        self.execute_cmd(command)
            .map_err(|e| { adbprint!("Error! {}", e); e })
    }

    pub fn execute_decl_file<P>(&mut self, filename: P) -> Result<(), io::Error>
    where
        P: AsRef<Path>, 
    {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Err(e) = self.wrapped_execute_cmd(&line?.replace("\n", "")) {
                adbprint!("Failed to execute line: {}", e);
            }
        }
        Ok(())
    }
}

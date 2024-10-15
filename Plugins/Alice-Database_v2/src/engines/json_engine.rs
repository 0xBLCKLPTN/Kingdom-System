use log::{ warn, info, error };

#[derive(Debug, Clone)]
pub struct JSONEngine {
    pub name: String,
    pub data: Vec<String>,
}

impl JSONEngine {
    pub fn new(name: String) -> Self {
        info!("Creating JSON Engine...");
        Self { name, data: Vec::new() }
    }
}

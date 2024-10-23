use uuid::Uuid;

pub fn get_files_in_dir() -> Vec<>

#[derive(Debug, Clone)]
pub struct JSONEngine {
    pub name: String,
    pub tables: Vec<String>,
}

impl JSONEngine {
    pub fn init() -> Self {
        let name = Uuid::new_v4().to_string();
    }
}

use std::error;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt; 

pub type BoxedResult<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type Databases = Vec<Database>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub ftype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub fields: Option<Vec<Field>>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub database_name: String,
    pub database_description: String,
    pub database_path: String,
    pub database_type: String,
    pub tables: Vec<Table>,
}
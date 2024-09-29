//!
//!    Filename: types.rs
//!    Description: Global types for AliceDB
//!    Author: Daniil (0xJanus) Ermolaev
//!

use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt; 

/// Default T or Error Boxed Result.
/// 
/// Examples
/// 
/// ```
/// pub fn some_function() -> BoxedResult<String> { ... }
/// pub fn another_function() -> BoxedResult<i32> { ... }
/// ```
pub type BoxedResult<T> = std::result::Result<T, Box<dyn Error>>;

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
    pub database_type: String,
    pub tables: Vec<Table>,
}
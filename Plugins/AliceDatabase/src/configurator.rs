use crate::atypes::*;
use std::fs;

pub fn read_config(file_path: &str) -> BoxedResult<Databases>
{
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let res: Databases = serde_json::from_str(&contents).expect("Should convert content into struct");
    return Ok(res);
}
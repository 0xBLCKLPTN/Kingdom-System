use crate::adbcore::types::{BoxedResult, Field};
use crate::adbcore::fs_manager::{simple_write, read_file};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use termion::color;

#[derive(Debug, Serialize, Deserialize)]
pub struct TablePickle {
    pub name: String,
    pub indexes: i32,
    pub fields: Vec<Field>,
}

pub async fn pickle_struct(data: &TablePickle) -> String {
    let data: Vec<u8> = bincode::serialize(data).unwrap();
    
    return match std::str::from_utf8(&data) {
        Ok(s) => {
            println!("Converted to &str: {}", s);
            String::from(s)
        },
        Err(e) => {
            String::from("Error")
        }
    }
}

pub async fn unpickle_struct(filepath: &PathBuf) -> BoxedResult<TablePickle> {
    adbprint!("Unpickle Struct");
    let encoded = read_file(filepath).await.unwrap().as_bytes().to_vec();
    let decoded: TablePickle = bincode::deserialize(&encoded[..]).unwrap();
    println!("{:#?}", decoded);
    Ok(decoded)
}
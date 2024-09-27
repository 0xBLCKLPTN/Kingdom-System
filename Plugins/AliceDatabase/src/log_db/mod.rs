///
///    FILE: //log_db/mod.rs
///    DESCRIPTION: AliceDB plugin that can write logs to file.
///

use crate::fs_manager::*;
use crate::atypes::*;
use std::path::PathBuf;

/// FUNCTION: create_log_table.
/// Simple write log data to file.
///
/// Usage example: 
///
/// ```rust
/// pub mod log_db;
/// use log_db::*;
/// let k = create_log_table(&str, &str);
/// ```

#[derive(Debug)]
pub struct LogDatabase {
    pub path: PathBuf,
    pub tables: Vec<LogTable>,
}
#[derive(Debug)]
pub struct LogTable {
    pub path: PathBuf,
    pub name: String,
}



impl LogDatabase {
    pub fn new(path: PathBuf) -> Self {
        let mut tables: Vec<LogTable> = Vec::new();
        create_dir(path.clone());
        return LogDatabase { path, tables }
    }

    pub async fn find_tables(&mut self) {
        let mut name = "";
        for table in list_dir(&self.path).unwrap() {
            self.tables.push(
                LogTable::add(
                    PathBuf::from(table.clone()),
                    pb_to_string(&table)
                        .as_str().split("/")
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string()
                )
            )
        }
    }

    pub async fn create_table(&mut self, table_name: &str) {
        LogTable::new(self.path.join(table_name), table_name.to_string());
    }

    pub async fn get_tables(&self) -> &Vec<LogTable>
    {
        return &self.tables;
    }
}

impl LogTable {
    pub fn new(path: PathBuf, name: String) -> Self {
        create_file(pb_to_string(&path) + ".alicedb.log");
        LogTable {path, name}
    }
    pub fn add(path: PathBuf, name: String) -> Self {
        LogTable { path, name }
    }

    pub async fn write(&self, data: &str) -> BoxedResult<()> {
        write_into_file(self.path.clone().into_os_string().into_string().unwrap(), data.to_string());
        Ok(())
    }
}

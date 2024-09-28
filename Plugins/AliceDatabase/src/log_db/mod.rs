use crate::fs_manager::*;
use crate::atypes::*;
use std::path::PathBuf;


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
    /// Creates a new `LogDatabase` instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the database directory.
    ///
    /// # Returns
    ///
    /// A new `LogDatabase` instance.
    pub fn new(path: PathBuf) -> Self {
        let mut tables: Vec<LogTable> = Vec::new();
        create_dir(path.clone());
        return LogDatabase { path, tables }
    }

    /// Finds and lists all tables within the database directory.
    ///
    /// This function populates the `tables` field with `LogTable` instances
    /// representing each table found in the directory.
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

    /// Creates a new table within the database.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to create.
    pub async fn create_table(&mut self, table_name: &str) {
        LogTable::new(self.path.join(table_name), table_name.to_string());
    }

    /// Returns a reference to the list of tables in the database.
    ///
    /// # Returns
    ///
    /// A reference to the `tables` field.
    pub async fn get_tables(&self) -> &Vec<LogTable>
    {
        return &self.tables;
    }
}

impl LogTable {
    /// Creates a new `LogTable` instance and the corresponding file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the table file.
    /// * `name` - The name of the table.
    ///
    /// # Returns
    ///
    /// A new `LogTable` instance.
    pub fn new(path: PathBuf, name: String) -> Self {
        create_file(pb_to_string(&path) + ".alicedb.log");
        LogTable {path, name}
    }

    /// Adds a new `LogTable` instance without creating a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the table file.
    /// * `name` - The name of the table.
    ///
    /// # Returns
    ///
    /// A new `LogTable` instance.
    pub fn add(path: PathBuf, name: String) -> Self {
        LogTable { path, name }
    }

    /// Writes data to the table file.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the file.
    ///
    /// # Returns
    ///
    /// A `BoxedResult` indicating success or failure.
    pub async fn write(&self, data: &str) -> BoxedResult<()> {
        write_into_file(pb_to_string(&self.path), data.to_string());
        Ok(())
    }
}

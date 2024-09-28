use std::path::PathBuf;
use crate::atypes::*;
use crate::fs_manager::*;

#[derive(Debug)]
pub struct DField {
    pub name: String,
    pub ftype: String,
}

#[derive(Debug)]
pub struct FillField {
    pub name: String,
    pub data: String,
}

#[derive(Debug)]
pub struct DTable {
    pub name: String,
    pub fields: Vec<DField>,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct DefaultDatabase {
    pub path: PathBuf,
    pub tables: Vec<DTable>,
}

impl DTable {
    pub fn new(name: &str, fields: Vec<DField>, path: PathBuf) -> DTable {
        create_file(pb_to_string(&path) + ".alicedb.dtable");
        DTable { name: name.to_string(), fields, path }
    }
    pub fn add(path: PathBuf, name: String) -> Self {
        let mut fields: Vec<DField> = Vec::new();
        DTable {name: name.to_string(), path, fields}
    }
    pub async fn write(&self, fields_data: Vec<String>) {
        let mut data: String = "".to_string();
        for k in fields_data {
            data += &k.to_string();
        }
        write_into_file(pb_to_string(&self.path), data.to_string());
    }
}

impl DField {
    pub fn new(name: &str, ftype: &str) -> DField {
        DField {name: name.to_string(), ftype: ftype.to_string() }
    }
}

impl DefaultDatabase {
    pub fn new(path: PathBuf) -> DefaultDatabase {
        let mut tables: Vec<DTable> = Vec::new();
        create_dir(path.clone());
        return DefaultDatabase { path, tables}
    }

    pub async fn find_tables(&mut self) {
        let mut name = "";
        for table in list_dir(&self.path).unwrap() {
            self.tables.push(
                DTable::add(
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

    pub async fn get_table_by_name(&self, name: &str) -> &DTable {
        for table in &self.tables {
            if table.name == name.to_string() {
                return &table;
            }
        }
        return &self.tables[0];
    }

    pub async fn create_table(&mut self, table_name: &str, fields: Vec<DField>) {
        let table = DTable::new(table_name, fields,  self.path.join(table_name));
        self.tables.push(table);
    }
}
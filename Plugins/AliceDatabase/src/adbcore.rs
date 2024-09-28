use crate::atypes::BoxedResult;
use crate::pb_to_string;
use crate::create_dir;
use crate::PathBuf;
use crate::list_dir;
use crate::create_file;
use crate::write_into_file;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ftype: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub database: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub path: PathBuf,
    pub name: String,
    pub tables: Vec<Table>,
    pub engine: String,

}

pub trait DatabaseController {
    fn new(path_to_database: PathBuf, engine: &str) -> Self;
    async fn simple_write(&self, data: String) -> BoxedResult<()> {
        todo!()
    } 
    async fn create_table(&mut self, table_name: &str) -> BoxedResult<()>;
    async fn find_tables(&mut self) -> BoxedResult<()>;
    async fn write(&self, table_name: &str, data: &str) -> BoxedResult<()>;
    async fn get_table_by_name(&self, name: String) -> Option<&Table>;


}
#[derive(Debug)]
pub struct Databases {
    pub databases: Vec<Database>,
}

impl Databases {
    pub fn new() -> Databases {
        let databases: Vec<Database> = Vec::new();
        Databases { databases }
    }
    pub fn add(&mut self, mut db: Database) {
        self.databases.push(db);
    }

    pub fn get_db_by_name(&self, name: String) -> Option<&Database> {
        for k in &self.databases {
            if k.name == name {
                return Some(&k);
            }
        }
        return None;
    }
}

impl DatabaseController for Database {
    fn new(path_to_database: PathBuf, engine: &str) -> Database {
        let mut tables: Vec<Table> = Vec::new();
        create_dir(path_to_database.clone());
        let name = pb_to_string(&path_to_database.clone()).as_str().split("/").collect::<Vec<_>>().last().unwrap().to_string();
        return Database { path: path_to_database, tables, name, engine: engine.to_string()}
    }

    async fn create_table(&mut self, table_name: &str) -> BoxedResult<()> {
        create_file(pb_to_string(&self.path) + "/" + table_name +"." + &self.engine + ".adb");
        Ok(
            self.tables.push(
                Table {
                    name: table_name.to_string(),
                    database: self.name.clone(),
                }
            )
        )
    }

    async fn find_tables(&mut self) -> BoxedResult<()> {
        let mut name = "";
        for table in list_dir(&self.path).unwrap() {
            self.tables.push(
                Table {
                    name: pb_to_string(&table)
                        .as_str()
                        .split("/")
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string(),
                    database: self.name.clone(),
                }
            )
        }
        Ok(())
    }

    async fn get_table_by_name(&self, name: String) -> Option<&Table> {
        for table in &self.tables {
            if table.name == name {
                return Some(&table);
            }
        }
        return None;
    }

    async fn write(&self, table_name: &str, data: &str) -> BoxedResult<()> {
        write_into_file(pb_to_string(&self.path) + "/" + table_name +"." + &self.engine + ".adb", data.to_string());
        Ok(())
    }
}
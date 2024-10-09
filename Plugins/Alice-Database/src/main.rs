use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs as async_fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{RwLock, RwLockReadGuard};
use log::{info, error, LevelFilter};
use simplelog::{Config, WriteLogger};
use std::io::ErrorKind;
use uuid::Uuid;
use serde_json;
use termion::color;
use tonic::{transport::Server, Request, Response, Status};

//use database::{database_service_server::{DatabaseService, DatabaseServiceServer}, CreateTableRequest, CreateTableResponse, InsertIntoRequest, InsertIntoResponse, FindInRequest, FindInResponse};

use database::{database_service_server::{DatabaseService, DatabaseServiceServer}, CreateTableRequest, CreateTableResponse, DropTableRequest, DropTableResponse, LoadTableRequest, LoadTableResponse, InsertIntoRequest, InsertIntoResponse, DeleteFromRequest, DeleteFromResponse, FindInRequest, FindInResponse, GetAllFromResponse, GetAllFromRequest};


pub mod database {
    tonic::include_proto!("database");
}

#[derive(Default)]
pub struct MyDatabaseService {
    db: Arc<RwLock<Database>>, // Changed to Arc<RwLock<Database>>
}
#[tonic::async_trait]
impl DatabaseService for MyDatabaseService {
    async fn create_table(&self, request: Request<CreateTableRequest>) -> Result<Response<CreateTableResponse>, Status> {
        let table_name = request.into_inner().name;

        let mut db = self.db.write().await;
        db.create_table(&table_name).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateTableResponse { message: format!("Table {} created", table_name) }))
    }

    async fn drop_table(&self, request: Request<DropTableRequest>) -> Result<Response<DropTableResponse>, Status> {
        let table_name = request.into_inner().name;

        let mut db = self.db.write().await;
        db.drop_table(&table_name).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DropTableResponse { message: format!("Table {} dropped", table_name) }))
    }

    async fn load_table(&self, request: Request<LoadTableRequest>) -> Result<Response<LoadTableResponse>, Status> {
        let table_name = request.into_inner().name;

        let mut db = self.db.write().await;
        db.load_table(&table_name).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(LoadTableResponse { message: format!("Table {} loaded", table_name) }))
    }

    async fn insert_into(&self, request: Request<InsertIntoRequest>) -> Result<Response<InsertIntoResponse>, Status> {
        let req = request.into_inner();
        let table_name = req.table_name;
        let data = req.data; // Adjust the data parsing if needed

        let mut db = self.db.write().await;
        db.insert_into(&table_name, data).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(InsertIntoResponse { message: "Insert successful".into() }))
    }

    async fn delete_from(&self, request: Request<DeleteFromRequest>) -> Result<Response<DeleteFromResponse>, Status> {
        let req = request.into_inner();
        let table_name = req.table_name;
        let id = req.id;

        let mut db = self.db.write().await;
        db.delete_from(&table_name, &id).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteFromResponse { message: "Delete successful".into() }))
    }

    async fn find_in(&self, request: Request<FindInRequest>) -> Result<Response<FindInResponse>, Status> {
        let req = request.into_inner();
        let table_name = req.table_name;
        let id = req.id;

        let db = self.db.read().await;
        match db.find_in(&table_name, &id).await {
            Ok(Some(item)) => Ok(Response::new(FindInResponse { data: serde_json::to_string(&item).unwrap(), message: "Item found".into() })),
            Ok(None) => Ok(Response::new(FindInResponse { data: "".into(), message: "Item not found".into() })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
    async fn get_all_from(&self, request: Request<GetAllFromRequest>) -> Result<Response<GetAllFromResponse>, Status> {

        let table_name = request.into_inner().table_name;


        let db = self.db.read().await;

        match db.get_all(&table_name).await {

            Ok(generic_items) => {

                // Map GenericItem to String (e.g., assuming generic_items is Vec<GenericItem>)

                let items: Vec<String> = generic_items.iter().map(|item| item.data.clone()).collect();

                Ok(Response::new(GetAllFromResponse {

                    data: items,

                    message: "Items retrieved successfully".into(),

                }))

            },

            Err(e) => Err(Status::internal(e.to_string())),

        }

    }
}



fn print_ascii() {
    println!(r"
    @---------------------------------------------------------------@
    |       {} ______     __         __     ______     ______{}         |
    |       {}/\  __ \   /\ \       /\ \   /\  ___\   /\  ___\{}        |
    |       {}\ \  __ \  \ \ \____  \ \ \  \ \ \____  \ \  __\{}        |
    |        {}\ \_\ \_\  \ \_____\  \ \_\  \ \_____\  \ \_____\{}      |
    |         {}\/_/\/_/   \/_____/   \/_/   \/_____/   \/_____/{}      |
    |                                                               |
    |                    {} _____     ______{}                          |
    |                    {}/\  __-.  /\  == \{}                         |
    |                    {}\ \ \/\ \ \ \  __<{}                         |
    |                     {}\ \____-  \ \_____\{}                       |
    |                      {}\/____/   \/_____/{}                       |
    |                                                               |
    @---------------------------------------------------------------@
    ",
    color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
             color::Fg(color::Yellow), color::Fg(color::Reset),
    );
}

macro_rules! adbprint {
    ($($arg:tt)*) => {
        println!("[ {}ADB{} ]=: {}", color::Fg(color::Yellow), color::Fg(color::Reset), format!($($arg)*));
    };
}

#[derive(Debug)]
pub enum DatabaseError {
    IoError(io::Error),
    SerializationError(serde_json::Error),
    NotFound(String),
    TableAlreadyExists(String),
    TableDoesNotExist(String),
    TonicTransportError(tonic::transport::Error), // Added
}

impl From<tonic::transport::Error> for DatabaseError {
    fn from(err: tonic::transport::Error) -> Self {
        DatabaseError::TonicTransportError(err)
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DatabaseError {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GenericItem {
    id: String,
    data: String,
}

impl GenericItem {
    pub fn new<T: Serialize>(data: T) -> Result<Self, DatabaseError> {
        let id = Uuid::new_v4().to_string();
        let data = serde_json::to_string(&data).map_err(DatabaseError::SerializationError)?;
        Ok(Self { id, data })
    }
}

pub struct Table {
    name: String,
    items: HashMap<String, GenericItem>, // Хранит элементы таблицы
}

impl Table {
    pub fn new(name: &str) -> Self {
        Table {
            name: name.to_string(),
            items: HashMap::new(),
        }
    }

    pub async fn load(&mut self, path: &Path) -> Result<(), DatabaseError> {
        let contents = async_fs::read_to_string(path).await.map_err(DatabaseError::IoError)?;
        self.items = serde_json::from_str(&contents).map_err(DatabaseError::SerializationError)?;
        Ok(())
    }

    pub async fn save(&self, path: &Path) -> Result<(), DatabaseError> {
        let serialized = serde_json::to_string(&self.items).map_err(DatabaseError::SerializationError)?;
        async_fs::write(path, serialized).await.map_err(DatabaseError::IoError)?;
        Ok(())
    }

    pub async fn insert<T: Serialize>(&mut self, data: T) -> Result<(), DatabaseError> {
        let item = GenericItem::new(data)?;
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    pub async fn delete(&mut self, id: &str) -> Result<(), DatabaseError> {
        if self.items.remove(id).is_none() {
            return Err(DatabaseError::NotFound(id.to_string()));
        }
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Option<&GenericItem> {
        self.items.get(id)
    }
}
#[derive(Default)]
pub struct Database {
    path: PathBuf,
    tables: HashMap<String, Table>, // Хранит таблицы
    cache: RwLock<HashMap<String, GenericItem>>, // Кэш
}

impl Database {
    pub fn new(path: &str) -> Self {
        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            std::fs::create_dir_all(&path_buf).expect("Failed to create database directory");
        }
        Database {
            path: path_buf,
            tables: HashMap::new(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create_table(&mut self, name: &str) -> Result<(), DatabaseError> {
        if self.tables.contains_key(name) {
            return Err(DatabaseError::TableAlreadyExists(name.to_string()));
        }

        self.tables.insert(name.to_string(), Table::new(name));
        let table_path = self.path.join(format!("{}.json", name));
        if let Err(e) = async_fs::File::create(&table_path).await {
            return Err(DatabaseError::IoError(e));
        }
        Ok(())
    }

    pub async fn drop_table(&mut self, name: &str) -> Result<(), DatabaseError> {
        if self.tables.remove(name).is_none() {
            return Err(DatabaseError::TableDoesNotExist(name.to_string()));
        }
        let table_path = self.path.join(format!("{}.json", name));
        if let Err(e) = async_fs::remove_file(&table_path).await {
            return Err(DatabaseError::IoError(e));
        }
        Ok(())
    }

    pub async fn load_table(&mut self, name: &str) -> Result<(), DatabaseError> {
        let table_path = self.path.join(format!("{}.json", name));
        let mut table = Table::new(name);
        table.load(&table_path).await?;
        self.tables.insert(name.to_string(), table);
        Ok(())
    }

    pub async fn insert_into(&mut self, table_name: &str, data: impl Serialize) -> Result<(), DatabaseError> {
        if let Some(table) = self.tables.get_mut(table_name) {
            table.insert(data).await?;
            table.save(&self.path.join(format!("{}.json", table_name))).await?;
            let item_id = Uuid::new_v4().to_string(); // Получаем ID элемента
            {
                // Записываем в кэш
                let mut cache = self.cache.write().await;
                if let Some(item) = table.items.get(&item_id) {
                    cache.insert(item_id.clone(), item.clone());
                } else {
                    // Обработка случая, когда элемент не найден
                    adbprint!("Item with ID {} not found in table items.", item_id);
                }
            }
            Ok(())
        } else {
            Err(DatabaseError::TableDoesNotExist(format!("Table {} does not exist", table_name)))
        }
    }
    pub async fn get_all(&self, table_name: &str) -> Result<Vec<GenericItem>, DatabaseError> {

        if let Some(table) = self.tables.get(table_name) {

            Ok(table.items.values().cloned().collect()) // Clone values and return as Vec

        } else {

            Err(DatabaseError::TableDoesNotExist(format!("Table {} does not exist", table_name)))

        }

    }

    pub async fn delete_from(&mut self, table_name: &str, id: &str) -> Result<(), DatabaseError> {
        if let Some(table) = self.tables.get_mut(table_name) {
            table.delete(id).await?;
            table.save(&self.path.join(format!("{}.json", table_name))).await?;
            {
                // Удаляем из кэша
                let mut cache = self.cache.write().await;
                cache.remove(id);
            }
            Ok(())
        } else {
            Err(DatabaseError::TableDoesNotExist(format!("Table {} does not exist", table_name)))
        }
    }

    pub async fn find_in(&self, table_name: &str, id: &str) -> Result<Option<GenericItem>, DatabaseError> {
        {
            // Попробуем найти в кэше
            let cache = self.cache.read().await;
            if let Some(item) = cache.get(id) {
                return Ok(Some(item.clone())); // Клонируем элемент перед возвратом
            }
        }

        if let Some(table) = self.tables.get(table_name) {
            Ok(table.find_by_id(id).await.cloned()) // Клонируем элемент из таблицы перед возвращением
        } else {
            Err(DatabaseError::TableDoesNotExist(format!("Table {} does not exist", table_name)))
        }
    }
    pub async fn load_all_tables(&mut self) -> Result<(), DatabaseError> {
        let mut entries = async_fs::read_dir(&self.path)
        .await
        .map_err(DatabaseError::IoError)?;

        while let Some(entry) = entries.next_entry().await.map_err(DatabaseError::IoError)? {
            let path = entry.path();

            if path.extension().and_then(std::ffi::OsStr::to_str) == Some("json") {
                let table_name = path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or_else(|| DatabaseError::IoError(io::Error::new(ErrorKind::Other, "Invalid file name")))?;

                let mut table = Table::new(table_name);
                table.load(&path).await?;
                self.tables.insert(table_name.to_string(), table);
            }
        }

        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_create_table() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut db = Database::new("test_database");
            db.create_table("test_table").await.unwrap();
            assert!(db.tables.contains_key("test_table"));
        });
    }

    #[test]
    fn test_insert_item() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut db = Database::new("test_database");
            db.create_table("test_table").await.unwrap();
            let item_id = "item1".to_string();
            db.insert_into("test_table", "Item 1").await.unwrap();
            assert!(db.tables["test_table"].items.contains_key(&item_id));
        });
    }

    #[test]
    fn test_delete_item() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut db = Database::new("test_database");
            db.create_table("test_table").await.unwrap();
            let item_id = "item1".to_string();
            db.insert_into("test_table", "Item 1").await.unwrap();
            db.delete_from("test_table", &item_id).await.unwrap();
            let result = db.find_in("test_table", &item_id).await.unwrap();
            assert!(result.is_none());
        });
    }

    #[test]
    fn test_find_nonexistent_item() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut db = Database::new("test_database");
            db.create_table("test_table").await.unwrap();
            let item_id = "nonexistent_item".to_string();
            let result = db.find_in("test_table", &item_id).await.unwrap();
            assert!(result.is_none());
        });
    }
}

/*
#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    print_ascii();
    let log_file = File::create("app.log").map_err(DatabaseError::IoError)?;

    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).map_err(|e| {
        DatabaseError::IoError(std::io::Error::new(ErrorKind::Other, e.to_string()))
    })?;

    let mut db = Database::new("database");

    db.create_table("people").await?;
    db.insert_into("people", "Alice").await?;
    db.insert_into("people", "Bob").await?;

    // Получаем id элемента, если они существуют
    let item_id = {
        let table = db.tables.get("people").unwrap(); // Здесь мы уверены, что таблица существует
        if let Some(first_id) = table.items.keys().next() {
            first_id.clone() // Если есть, берем первый id
        } else {
            adbprint!("No items found in the table."); // Если таблица пуста
            return Ok(()); // Завершаем выполнение
        }
    };

    match db.find_in("people", &item_id).await {
        Ok(Some(item)) => {
            adbprint!("Found item: {:#?}", item);
        }
        Ok(None) => {
            adbprint!("Item with ID {} not found.", item_id);
        }
        Err(e) => {
            adbprint!("Error finding item: {}", e);
        }
    }

    // Удаление элемента
    db.delete_from("people", &item_id).await?;
    adbprint!("Deleted item with ID: {}", item_id);

    Ok(())
}
*/
#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    // Your existing database initialization code here...
    print_ascii();
    let log_file = File::create("app.log").map_err(DatabaseError::IoError)?;


    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).map_err(|e| {

        DatabaseError::IoError(std::io::Error::new(ErrorKind::Other, e.to_string()))

    })?;

    let db = Arc::new(RwLock::new(Database::new("database"))); // Wrap in Arc<RwLock<Database>>

    {

        let mut db_write = db.write().await;

        db_write.load_all_tables().await?; // Load existing tables

    }
    let addr = "[::1]:50051".parse().unwrap();
    let database_service = MyDatabaseService { db }; // Use the Arc<RwLock>

    info!("Server is starting on {:?}", addr);
    println!("Server is starting on {:?}", addr);
    Server::builder()
    .add_service(DatabaseServiceServer::new(database_service))
    .serve(addr)
    .await?;


    Ok(())
}

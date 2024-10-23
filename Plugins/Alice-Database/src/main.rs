use std::fs;
use std::io::{self, Read, Write};
use std::path::{PathBuf, Path};
use serde_json::{json, Value, Result as JsonResult};
use log::{info, error, trace};
use simplelog::*;
use chrono::Local;
use std::env; // Import this for getting the home directory

mod tests;
use tests::*;

// Define constants for the root path and log path
const ROOT_DIR: &str = "Alice-Database-Data";
const ADB_DATA_DIR: &str = "ADB_Data";
const JSON_ENGINE_DIR: &str = "json_engine";
const ADB_LOGS_DIR: &str = "ADB_Logs";

/// A struct representing a document in the database.
///
/// A `Document` contains its name, file path, and its content stored as a JSON `Value`.
#[derive(Debug)]
pub struct Document {
    pub name: String,
    pub path: PathBuf,
    pub json_value: Option<Value>,
}

/// A struct representing a collection of documents.
///
/// A `Collection` holds a name and a list of associated `Document`s.
#[derive(Debug)]
pub struct Collection {
    pub name: String,
    pub documents: Vec<Document>,
}

impl Collection {
    // Method to get a document by name
    pub fn get_document(&self, name: &str) -> Option<&Document> {
        self.documents.iter().find(|doc| doc.name == name)
    }

    /// Retrieve a mutable reference to a document by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the document to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a mutable reference to the `Document` if found,
    /// or `None` if not found.
    pub fn get_document_mut(&mut self, name: &str) -> Option<&mut Document> {
        self.documents.iter_mut().find(|doc| doc.name == name)
    }

    // Method to add a new document to the collection
    pub fn add_document(&mut self, name: &str, content: &str) -> io::Result<()> {
        let collection_path = Path::new(&self.path()).join(&self.name);
        fs::create_dir_all(&collection_path)?;

        let doc_path = collection_path.join(name); // Correctly construct the document path
        let mut file = fs::File::create(&doc_path)?;
        file.write_all(content.as_bytes())?;

        // Create a new Document instance
        let new_document = Document {
            name: name.to_string(),
            path: doc_path.clone(),
            json_value: parse_json_data(content).ok(),
        };
        self.documents.push(new_document);
        Ok(())
    }

    // Method to delete a document from the collection
    pub fn delete_document(&mut self, name: &str) -> io::Result<()> {
        if let Some(doc) = self.documents.iter().find(|doc| doc.name == name) {
            fs::remove_file(&doc.path)?;
            self.documents.retain(|doc| doc.name != name);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Document not found"))
        }
    }

    fn path(&self) -> PathBuf {
        let home_dir = env::home_dir().expect("Failed to get home directory");
        home_dir.join(ROOT_DIR).join(ADB_DATA_DIR).join(JSON_ENGINE_DIR)
    }
}

/// A struct to manage multiple collections of documents.
pub struct CollectionManager {
    collections: Vec<Collection>,
}

impl CollectionManager {
    /// Create a new `CollectionManager`.
    ///
    /// # Parameters
    /// - `root`: The path to the root directory for data storage.
    ///
    /// # Returns
    /// A new instance of `CollectionManager`.
    pub fn new(root: &Path) -> Self {
        let collections = get_exists_collections(root);
        CollectionManager { collections }
    }

    /// Retrieve a mutable reference to a collection by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the collection to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a mutable reference to the `Collection`, if found.
    pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.iter_mut().find(|col| col.name == name)
    }

    /// Add a new collection.
    ///
    /// # Parameters
    /// - `name`: The name of the collection to create.
    ///
    /// # Returns
    /// An `Option` containing a mutable reference to the newly added `Collection`.
    pub fn add_collection(&mut self, name: &str) -> Option<&mut Collection> {
        let collection_path = Path::new(&self.root_path()).join(name);
        fs::create_dir_all(&collection_path).ok()?; // Create the directory for new collection

        let new_collection = Collection {
            name: name.to_string(),
            documents: vec![],
        };

        self.collections.push(new_collection);
        self.collections.last_mut() // Return a mutable reference to the newly added collection
    }

    /// Get a collection by name.
    ///
    /// # Parameters
    /// - `name`: The name of the collection to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the `Collection`, if found.
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.iter().find(|col| col.name == name)
    }

    /// Get a document from a specific collection.
    ///
    /// # Parameters
    /// - `collection_name`: The name of the collection the document belongs to.
    /// - `document_name`: The name of the document to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the `Document`, if found.
    pub fn get_document(&self, collection_name: &str, document_name: &str) -> Option<&Document> {
        self.get_collection(collection_name)?.get_document(document_name)
    }

    fn root_path(&self) -> PathBuf {
        let home_dir = env::home_dir().expect("Failed to get home directory");
        home_dir.join(ROOT_DIR).join(ADB_DATA_DIR).join(JSON_ENGINE_DIR)
    }
}

impl Document {
    /// Update a field in the document.
    ///
    /// # Parameters
    /// - `key`: The key of the field to update.
    /// - `value`: The new value for the field.
    ///
    /// # Returns
    /// A result indicating success or failure.
    pub fn update_rows(&mut self, key: &str, value: &Value) -> io::Result<()> {
        if let Some(json_value) = &mut self.json_value {
            if let Some(obj) = json_value.as_object_mut() {
                obj.insert(key.to_string(), value.clone());
                let updated_content = serde_json::to_string_pretty(json_value)?;
                let mut file = fs::File::create(&self.path)?;
                file.write_all(updated_content.as_bytes())?;
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "JSON is not an object"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Document does not contain valid JSON"))
        }
    }

    /// Delete a field in the document.
    ///
    /// # Parameters
    /// - `key`: The key of the field to delete.
    ///
    /// # Returns
    /// A result indicating success or failure.
    pub fn delete_rows(&mut self, key: &str) -> io::Result<()> {
        if let Some(json_value) = &mut self.json_value {
            if let Some(obj) = json_value.as_object_mut() {
                obj.remove(key);
                let updated_content = serde_json::to_string_pretty(json_value)?;
                let mut file = fs::File::create(&self.path)?;
                file.write_all(updated_content.as_bytes())?;
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "JSON is not an object"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Document does not contain valid JSON"))
        }
    }

    /// Update a field in a nested JSON object.
    ///
    /// # Parameters
    /// - `parent_key`: The parent key of the nested field.
    /// - `key`: The key of the field to update within the parent key.
    /// - `value`: The new value for the nested field.
    ///
    /// # Returns
    /// A result indicating success or failure.
    pub fn update_nested_field(&mut self, parent_key: &str, key: &str, value: &Value) -> io::Result<()> {
        if let Some(json_value) = &mut self.json_value {
            if let Some(parent) = json_value.get_mut(parent_key) {
                if let Some(obj) = parent.as_object_mut() {
                    obj.insert(key.to_string(), value.clone());
                    let updated_content = serde_json::to_string_pretty(json_value)?;
                    let mut file = fs::File::create(&self.path)?;
                    file.write_all(updated_content.as_bytes())?;
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Parent key is not an object"))
                }
            } else {
                Err(io::Error::new(io::ErrorKind::NotFound, "Parent key not found"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Document does not contain valid JSON"))
        }
    }
}

// Functions for handling file operations and collections
fn get_documents_in_collection(path: &Path) -> Vec<Document> {
    let entries = fs::read_dir(path).unwrap();
    let mut documents: Vec<Document> = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_file() {
            let name = entry_path.file_name().unwrap().to_string_lossy().into_owned();
            let data = read_file_data(&entry_path).unwrap_or_default();
            let json_value = parse_json_data(&data).ok();
            let document = Document {
                name,
                path: entry_path.clone(),
                json_value,
            };
            documents.push(document);
        }
    }
    documents
}

fn read_file_data(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_json_data(data: &str) -> JsonResult<Value> {
    serde_json::from_str(data)
}

fn get_exists_collections(path: &Path) -> Vec<Collection> {
    let mut collections: Vec<Collection> = vec![];

    if path.exists() && path.is_dir() {
        let entries = fs::read_dir(path).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let documents = get_documents_in_collection(&entry_path);
                let collection_name = entry_path.file_name().unwrap().to_string_lossy().into_owned();
                let collection = Collection {
                    name: collection_name,
                    documents,
                };
                collections.push(collection);
            }
        }
    } else {
        error!("The specified path does not exist or is not a directory: {:?}", path);
    }

    collections
}

/// The main entry point for the application.
fn main() -> std::io::Result<()> {
    // Get the home directory
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let base_path = home_dir.join(ROOT_DIR);
    let adb_data_path = base_path.join(ADB_DATA_DIR);
    let adb_logs_path = base_path.join(ADB_LOGS_DIR);

    // Ensure the base and log directories exist
    fs::create_dir_all(&adb_data_path).expect("Failed to create ADB_Data directory");
    fs::create_dir_all(&adb_logs_path).expect("Failed to create ADB_Logs directory");

    // Define the data path for JSON documents
    let root_path = adb_data_path.join(JSON_ENGINE_DIR);

    // Ensure the JSON engine directory exists
    fs::create_dir_all(&root_path).expect("Failed to create json_engine directory");

    // Generate a unique log filename using timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_path = format!("{}/{}.adb.log", adb_logs_path.display(), timestamp);

    // Set up logging configuration
    let log_config = ConfigBuilder::new().build();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, log_config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
                         WriteLogger::new(LevelFilter::Trace, log_config.clone(), fs::File::create(log_file_path).unwrap()),
        ]
    ).unwrap();

    trace!("Starting Collection Manager...");

    let mut manager = CollectionManager::new(&root_path);

    // Create a new collection
    let collection_name = "collection1"; // Example collection name
    if let Some(_) = manager.add_collection(collection_name) {
        trace!("Created collection: {}", collection_name);
    }

    // Create a new document within the created collection
    let document_name = "document5.json"; // Example document name
    let document_content = json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null
        }
    });

    // Convert JSON Value to String
    let document_content_str = serde_json::to_string(&document_content).expect("Failed to convert JSON to string");

    if let Some(collection) = manager.get_collection_mut(collection_name) {
        if let Err(e) = collection.add_document(document_name, &document_content_str) {
            error!("Failed to add document: {}", e);
        } else {
            trace!("Created document: {} in collection: {}", document_name, collection_name);
        }
    }

    // Example of accessing the document
    if let Some(doc) = manager.get_document(collection_name, document_name) {
        println!("Found document: {:?}", doc);
        trace!("Successfully found document: {}", doc.name);
    } else {
        println!("Document '{}' not found in collection '{}'", document_name, collection_name);
        error!("Document '{}' not found in collection '{}'", document_name, collection_name);
    }

    // Example of updating 'homepage' in the 'payload' field
    if let Some(doc) = manager.get_document_mut(collection_name, document_name) {
        // New value for 'payload.homepage'
        let new_homepage_value = json!("https://new-homepage-url.com");

        if let Err(e) = doc.update_nested_field("payload", "homepage", &new_homepage_value) {
            error!("Failed to update 'homepage' in 'payload': {}", e);
        } else {
            trace!("Updated 'homepage' in document: {}", document_name);
        }
    }

    Ok(())
}

// Helper method to get a mutable reference to a document
impl CollectionManager {
    pub fn get_document_mut(&mut self, collection_name: &str, document_name: &str) -> Option<&mut Document> {
        self.get_collection_mut(collection_name)?.documents.iter_mut().find(|doc| doc.name == document_name)
    }
}

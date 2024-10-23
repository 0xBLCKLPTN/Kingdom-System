<div align="center">
  <img src="https://github.com/0xBLCKLPTN/Kingdom-System/blob/alice_database-dev/Docs/illustrations/alice_db_logo.png"/>
</div>

# Alice Database

This module implements a simple collection and document management system for storing and manipulating JSON data on the filesystem. It utilizes Serde for JSON serialization/deserialization and logs operations for debugging purposes.

## Overview

The primary components of this module are `Document`, `Collection`, and `CollectionManager`. The `CollectionManager` handles collections of documents, while `Collection` manages individual documents. Documents are represented as JSON values, allowing for flexible data structures.

## Structures

### Document

A struct representing a document in the database.

#### Fields

- `name` (String): The name of the document.
- `path` (PathBuf): The file path to the document.
- `json_value` (Option<Value>): The content of the document stored as a JSON `Value`.

### Collection

A struct representing a collection of documents.

#### Fields

- `name` (String): The name of the collection.
- `documents` (Vec<Document>): A list of associated `Document`s.

### CollectionManager

A struct to manage multiple collections of documents.

#### Fields

- `collections` (Vec<Collection>): A list of collections managed by this manager.

## Methods

### Document Methods

#### `update_rows(key: &str, value: &Value) -> io::Result<()>`

Updates a field in the document.

- **Parameters:**
  - `key`: The key of the field to update.
  - `value`: The new value for the field.
- **Returns:** Result indicating success or failure.

#### `delete_rows(key: &str) -> io::Result<()>`

Deletes a field in the document.

- **Parameters:**
  - `key`: The key of the field to delete.
- **Returns:** Result indicating success or failure.

#### `update_nested_field(parent_key: &str, key: &str, value: &Value) -> io::Result<()>`

Updates a field in a nested JSON object.

- **Parameters:**
  - `parent_key`: The parent key of the nested field.
  - `key`: The key of the field to update within the parent key.
  - `value`: The new value for the nested field.
- **Returns:** Result indicating success or failure.

### Collection Methods

#### `get_document(name: &str) -> Option<&Document>`

Retrieves a document by name.

- **Parameters:**
  - `name`: The name of the document to retrieve.
- **Returns:** An `Option` containing a reference to the `Document` if found, or `None` if not found.

#### `get_document_mut(name: &str) -> Option<&mut Document>`

Retrieves a mutable reference to a document by its name.

- **Parameters:**
  - `name`: The name of the document to retrieve.
- **Returns:** An `Option` containing a mutable reference to the `Document` if found, or `None` if not found.

#### `add_document(name: &str, content: &str) -> io::Result<()>`

Adds a new document to the collection.

- **Parameters:**
  - `name`: The name of the document to create.
  - `content`: The content of the document to be written.
- **Returns:** A result indicating success or failure.

#### `delete_document(name: &str) -> io::Result<()>`

Deletes a document from the collection.

- **Parameters:**
  - `name`: The name of the document to delete.
- **Returns:** A result indicating success or failure.

### CollectionManager Methods

#### `new(root: &Path) -> Self`

Creates a new `CollectionManager`.

- **Parameters:**
  - `root`: The path to the root directory for data storage.
- **Returns:** A new instance of `CollectionManager`.

#### `get_collection_mut(name: &str) -> Option<&mut Collection>`

Retrieves a mutable reference to a collection by its name.

- **Parameters:**
  - `name`: The name of the collection to retrieve.
- **Returns:** An `Option` containing a mutable reference to the `Collection`, if found.

#### `add_collection(name: &str) -> Option<&mut Collection>`

Adds a new collection.

- **Parameters:**
  - `name`: The name of the collection to create.
- **Returns:** An `Option` containing a mutable reference to the newly added `Collection`.

#### `get_collection(name: &str) -> Option<&Collection>`

Gets a collection by name.

- **Parameters:**
  - `name`: The name of the collection to retrieve.
- **Returns:** An `Option` containing a reference to the `Collection`, if found.

#### `get_document(collection_name: &str, document_name: &str) -> Option<&Document>`

Gets a document from a specific collection.

- **Parameters:**
  - `collection_name`: The name of the collection the document belongs to.
  - `document_name`: The name of the document to retrieve.
- **Returns:** An `Option` containing a reference to the `Document`, if found.

## Usage

Here's a brief example of how to use the module:

```rust
fn main() -> std::io::Result<()> {
    let root_path = Path::new("path_to_your_database");
    let mut manager = CollectionManager::new(&root_path);
    
    manager.add_collection("example_collection").unwrap();
    let doc_content = json!({"key": "value"}).to_string();
    manager.get_collection_mut("example_collection").unwrap().add_document("example_doc.json", &doc_content).unwrap();
}


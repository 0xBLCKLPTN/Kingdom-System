use crate::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;
    use serde_json::json;
    use super::*;
    const TEST_ROOT_DIR: &str = "Alice-Database-Data-Test";
    const TEST_ADB_DATA_DIR: &str = "ADB_Data";
    const TEST_JSON_ENGINE_DIR: &str = "json_engine";

    // Setup a temporary test directory
    fn setup_test_dir() -> PathBuf {
        let home_dir = env::temp_dir();
        let test_dir = home_dir.join(TEST_ROOT_DIR);
        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        test_dir
    }

    #[test]
    fn test_create_and_add_document() {
        let root_path = setup_test_dir();
        let mut manager = CollectionManager::new(&root_path);

        // Create a collection
        manager.add_collection("test_collection").unwrap();
        let collection = manager.get_collection_mut("test_collection").unwrap();

        // Add a document
        let doc_content = json!({"code": 200, "success": true}).to_string();
        let result = collection.add_document("test_document.json", &doc_content);
        assert!(result.is_ok());

        // Verify the document is added
        assert_eq!(collection.documents.len(), 1);
        assert_eq!(collection.documents[0].name, "test_document.json");
    }

    #[test]
    fn test_update_document() {
        let root_path = setup_test_dir();
        let mut manager = CollectionManager::new(&root_path);

        // Create a collection and add a document
        manager.add_collection("test_collection").unwrap();
        let collection = manager.get_collection_mut("test_collection").unwrap();
        let doc_content = json!({
            "code": 200,
            "success": true,
            "payload": {
                "homepage": null
            }
        }).to_string();
        collection.add_document("test_document.json", &doc_content).unwrap();

        // Update the homepage field
        let doc = collection.get_document_mut("test_document.json").unwrap();
        let new_homepage_value = json!("https://new-homepage-url.com");

        let update_result = doc.update_nested_field("payload", "homepage", &new_homepage_value);
        assert!(update_result.is_ok());

        // Check the updated value
        assert_eq!(doc.json_value.as_ref().unwrap()["payload"]["homepage"], new_homepage_value);
    }

    #[test]
    fn test_delete_document() {
        let root_path = setup_test_dir();
        let mut manager = CollectionManager::new(&root_path);

        // Create a collection and add a document
        manager.add_collection("test_collection").unwrap();
        let collection = manager.get_collection_mut("test_collection").unwrap();
        collection.add_document("test_document.json", "{\"key\":\"value\"}").unwrap();

        // Ensure the document exists before deletion
        assert_eq!(collection.documents.len(), 1);

        // Delete the document
        let delete_result = collection.delete_document("test_document.json");
        assert!(delete_result.is_ok());

        // Verify the document was deleted
        assert_eq!(collection.documents.len(), 0);
    }

}

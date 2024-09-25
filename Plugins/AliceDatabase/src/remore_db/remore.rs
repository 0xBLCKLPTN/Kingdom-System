use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheEntry {
    value: String,
}


pub struct RemoreDBInstance {
    pub name: String,
    pub host: String,
    pub port: u8,
    pub key: String,
    pub storage: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

pub struct RemoreInstances {
    pub local_instance: RemoreDBInstance,
    pub remote_instances: Vec<RemoreDBInstance>,
}

impl RemoreDBInstance {
    pub fn new(name: String, host: Option<String>, port: Option<u8>, key: String) -> Self {
        let mut storage = Arc::new(RwLock::new(HashMap::new()));
        RemoreDBInstance { name, host: host.unwrap(), port: port.unwrap(), key, storage }
    }

    async fn get(&self, key: &str) -> Option<CacheEntry> {
        let storage = self.storage.read().await;
        storage.get(key).cloned()
    }

    async fn set(&self, key: String, value: CacheEntry) {
        let mut storage = self.storage.write().await;
        storage.insert(key, value);
    }

    async fn delete(&self, key: &str) -> Option<CacheEntry> {
        let mut storage = self.storage.write().await;
        storage.remove(key)
    }
}
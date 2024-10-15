use log::{ info, error };
use uuid::Uuid;

use tokio::{ fs::{ self, File as AsyncFile }, io::AsyncWriteExt };

#[derive(Debug, Clone)]
pub struct LogEngine {
    pub name: String,
    pub logs: Vec<String>,
}

impl LogEngine
{
    pub fn new(name: String) -> Self {
        Self { name, logs: Vec::new() }
    }

    pub async fn write(&mut self, instance_name: &str, data: &str) {
        let filepath = format!("ADB_Data/log_engine/{}/{}.alicedb.log", instance_name, self.name);

        match AsyncFile::options().append(true).create(true).open(&filepath).await
        {
            Ok(mut file) =>
            {
                if let Err(e) = file.write_all( (data.to_owned() + "\n").as_bytes() ).await {
                    error!("Failed to write data to {}: {}", filepath, e);
                } else {
                    info!("Data written to {}", filepath);
                }
            },
            Err(e) => {
                error!("Failed to open file {}: {}", filepath, e);
            }
        }
    }
}

pub mod instance_manager;
pub mod engines;


use engines::EngineType;

#[cfg(feature = "log_engine")]
use engines::log_engine::LogEngine;

#[cfg(feature = "json_engine")]
use engines::json_engine::JSONEngine;

use instance_manager::ADBInstanceManager;

use log::{ info, error };


#[tokio::main]
async fn main() {
	let mut instance_manager = match ADBInstanceManager::find_all().await {
		Ok(manager) => manager,
		Err(e) => {
			error!("Failed to find instances: {}", e);
			info!("Creating new instance manager ...");
			ADBInstanceManager::new().await
		},
	};
	instance_manager.create_new_instance("json_engine").await;
	println!("Instance Manager: {:#?}", instance_manager);
	println!("Hello World!");
}


// ===================
/*
 *   FILENAME: root/instance_manager.rs
 *   AUTHOR: Daniil (0xJanus) Ermolaev.
 *   ----------------------------------
 *   DESCRIPTION:
 *       The instance manager is an essential part of the entire database management system.
 *   It creates instances (ADBInstance), which create database engines, engines are tables into which
 *   you write any data in any format.
 *
 *       Basically, ADBInstanceManager is needed for interaction and management with ADBInstance,
 *   it looks like a kind of wrapper over all of them and somehow tries to orchestrate them all.
 */
// ==================

use uuid::Uuid;
use log::{ info, error, trace, warn };

use crate::engines::EngineType;
#[cfg(feature="log_engine")]
use crate::LogEngine;

#[cfg(feature="json_engine")]
use crate::JSONEngine;

use tokio::{ io::{ Result as TResult, AsyncWriteExt},
fs::{ self, File as AsyncFile },
};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use tokio::task;


#[derive(Clone, Debug)]
pub struct ADBInstance
{
    pub name: String,
    pub engine: EngineType,
}

impl ADBInstance
{
    pub async fn new(engine_name: &str) -> Self
    {
        trace!("Creating new ADBInstance");
        let engine = match engine_name {
            #[cfg(feature = "log_engine")]
            "log_engine" => {
                let name = Uuid::new_v4().to_string();
                let filepath = format!("ADB_Data/log_engine/{}", name);
                fs::create_dir_all(filepath).await.expect("Failed to create main log_engine");
                EngineType::Log(LogEngine::new(name))
            },
            #[cfg(feature="json_engine")]
            "json_engine" => {
                let name = Uuid::new_v4().to_string();
                let filepath = format!("ADB_Data/json_engine/{}", name);
                fs::create_dir_all(filepath).await.expect("Failed to create main json_engine");
                EngineType::JSON(JSONEngine::new(name))
            },
            _ => {
                warn!("No engine selected! Run panic!");
                panic!("No engine selected or you select not implemented engine!");
            },
        };

        info!("ADBInstance created!");
        Self { name: Uuid::new_v4().to_string(), engine }
    }
    pub async fn create_from_exist(name: String, engine_type: String) -> Self
    {
        trace!("Creating new ADBInstance from exist...");

        let engine = match engine_type.as_str() {
            #[cfg(feature = "log_engine")]
            "log_engine" => EngineType::Log(LogEngine::new(name.clone())),
            #[cfg(feature = "json_engine")]
            "json_engine" => EngineType::JSON(JSONEngine::new(name.clone())),
            _ => {
                warn!("No engine selected! Run panic!");
                panic!("No engine selected or you select not implemented engine!");
            },
        };

        info!("ADBInstance created!");
        Self { name, engine }
    }
}

#[derive(Debug, Clone)]
pub struct ADBInstanceManager
{
    pub name: String,
    pub instances: Vec<ADBInstance>,
}

impl ADBInstanceManager
{
    pub async fn new() -> Self
    {
        fs::create_dir_all("ADB_Data").await.expect("Failed to create main ADB_Data directory.");
        trace!("Instance Manager created!");
        let mut instances: Vec<ADBInstance> = vec![];
        Self { name: Uuid::new_v4().to_string(), instances: Vec::new() }
    }


    pub async fn find_all() -> TResult<Self> {
        let mut instances = Vec::new();
        let paths = std::fs::read_dir("ADB_Data/")?;

        for entry in paths.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let engine_type = path.file_name().unwrap().to_string_lossy().to_string();
                let instance_path = std::fs::read_dir(path)?;

                for instance_entry in instance_path.filter_map(Result::ok) {
                    let instance_name = instance_entry.file_name().to_string_lossy().to_string();
                    let instance = ADBInstance::create_from_exist(instance_name, engine_type.clone()).await;
                    instances.push(instance);
                }
            }
        }
        info!("Instance manager found all ADBInstances and created them.");

        // Return a new instance of ADBInstanceManager with the collected instances
        Ok(Self { name: Uuid::new_v4().to_string(), instances })
    }


    pub async fn get_instance(&mut self, instance_name: &str) -> Option<&mut ADBInstance> {
        self.instances.iter_mut().find(| instance | instance.name == instance_name )
    }
    pub async fn create_new_instance(&mut self, engine_type: &str) {
        self.instances.push(ADBInstance::new(engine_type).await)

    }

}

// C bindings

#[no_mangle]
pub extern "C" fn adb_instance_new(engine_name: *const c_char) -> *mut ADBInstance {
    let c_str = unsafe { CStr::from_ptr(engine_name) };
    let rust_str = c_str.to_str().unwrap();

    let instance = task::block_in_place(|| {
        // Call the async function in a blocking context
        tokio::runtime::Runtime::new().unwrap().block_on(ADBInstance::new(rust_str))
    });

    Box::into_raw(Box::new(instance))
}

#[no_mangle]
pub extern "C" fn adb_instance_manager_new() -> *mut ADBInstanceManager {
    let manager = task::block_in_place(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(ADBInstanceManager::new())
    });

    Box::into_raw(Box::new(manager))
}

#[no_mangle]
pub extern "C" fn adb_instance_manager_find_all(manager: *mut ADBInstanceManager) -> *mut ADBInstanceManager {
    let new_manager = tokio::runtime::Runtime::new().unwrap().block_on(ADBInstanceManager::find_all());

    match new_manager {
        Ok(manager) => Box::into_raw(Box::new(manager)), // return new manager pointer
        Err(_) => std::ptr::null_mut(), // handle the error case
    }
}



#[no_mangle]
pub extern "C" fn adb_instance_manager_create_new_instance(manager: *mut ADBInstanceManager, engine_type: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(engine_type) };
    let rust_str = c_str.to_str().unwrap();

    task::block_in_place(|| {
        let manager_ref = unsafe { &mut *manager };
        tokio::runtime::Runtime::new().unwrap().block_on(manager_ref.create_new_instance(rust_str));
    });
}

#[no_mangle]
pub extern "C" fn adb_instance_free(instance: *mut ADBInstance) {
    if !instance.is_null() {
        unsafe { Box::from_raw(instance) };  // Automatically cleaned up
    }
}

#[no_mangle]
pub extern "C" fn adb_instance_manager_free(manager: *mut ADBInstanceManager) {
    if !manager.is_null() {
        unsafe { Box::from_raw(manager) };  // Automatically cleaned up
    }
}

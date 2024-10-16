use log::{info, error};
use uuid::Uuid;
use tokio::{fs::{self, File as AsyncFile}, io::AsyncWriteExt};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// LogEngine struct definition
#[derive(Debug, Clone)]
pub struct LogEngine {
    pub name: String,
    pub logs: Vec<String>,
}

impl LogEngine {
    pub fn new(name: String) -> Self {
        Self { name, logs: Vec::new() }
    }

    pub async fn write(&mut self, instance_name: &str, data: &str) {
        let filepath = format!("ADB_Data/log_engine/{}/{}.alicedb.log", instance_name, self.name);

        match AsyncFile::options().append(true).create(true).open(&filepath).await {
            Ok(mut file) => {
                if let Err(e) = file.write_all((data.to_owned() + "\n").as_bytes()).await {
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

// C bindings
#[no_mangle]
pub extern "C" fn log_engine_new(name: *const c_char) -> *mut LogEngine {
    let c_str = unsafe { CStr::from_ptr(name) };
    let rust_str = c_str.to_str().unwrap();
    Box::into_raw(Box::new(LogEngine::new(rust_str.to_string())))
}

#[no_mangle]
pub extern "C" fn log_engine_write(engine_ptr: *mut LogEngine, instance_name: *const c_char, data: *const c_char) {
    let engine = unsafe { &mut *engine_ptr };
    let c_instance_str = unsafe { CStr::from_ptr(instance_name) };
    let c_data_str = unsafe { CStr::from_ptr(data) };

    let rust_instance_name = c_instance_str.to_str().unwrap();
    let rust_data = c_data_str.to_str().unwrap();

    // Blocking context to call the async method
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(async {
        engine.write(rust_instance_name, rust_data).await;
    });
}

#[no_mangle]
pub extern "C" fn log_engine_free(engine_ptr: *mut LogEngine) {
    if engine_ptr.is_null() { return; }
    unsafe { Box::from_raw(engine_ptr) };  // Automatically cleaned up
}

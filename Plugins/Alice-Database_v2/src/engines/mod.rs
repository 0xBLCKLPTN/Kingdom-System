use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(feature = "log_engine")]
pub mod log_engine;
#[cfg(feature = "json_engine")]
pub mod json_engine;

#[cfg(feature = "log_engine")]
use log_engine::LogEngine;
#[cfg(feature = "json_engine")]
use json_engine::JSONEngine;

#[derive(Debug, Clone)]
pub enum EngineType {
    #[cfg(feature = "log_engine")]
    Log(LogEngine),
    #[cfg(feature = "json_engine")]
    JSON(JSONEngine),
}

// C bindings for EngineType
#[no_mangle]
#[cfg(feature = "log_engine")]
pub extern "C" fn engine_type_create_log(name: *const c_char) -> *mut EngineType {
    let c_str = unsafe { CStr::from_ptr(name) };
    let rust_str = c_str.to_str().unwrap();
    let log_engine = LogEngine::new(rust_str.to_string());
    Box::into_raw(Box::new(EngineType::Log(log_engine)))
}

#[no_mangle]
#[cfg(feature = "json_engine")]
pub extern "C" fn engine_type_create_json(name: *const c_char) -> *mut EngineType {
    let c_str = unsafe { CStr::from_ptr(name) };
    let rust_str = c_str.to_str().unwrap();
    let json_engine = JSONEngine::new(rust_str.to_string());
    Box::into_raw(Box::new(EngineType::JSON(json_engine)))
}

#[no_mangle]
pub extern "C" fn engine_type_free(engine_ptr: *mut EngineType) {
    if engine_ptr.is_null() { return; }
    unsafe { Box::from_raw(engine_ptr) };  // Automatically cleaned up
}

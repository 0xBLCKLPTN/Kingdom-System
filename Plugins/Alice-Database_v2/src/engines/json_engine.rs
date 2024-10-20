use log::{info, warn, error};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::collections::HashMap;

use serde_json::Value;
use serde_json::Map;

// Define the JSONEngine struct
#[derive(Debug, Clone)]
pub struct JSONEngine {
    pub name: String,
    pub data: Vec<Map<String, Value>>,
}

#[derive(Debug, Clone)]
pub struct Sort {
    pub data: Vec<HashMap<String, Value>>,
}

impl JSONEngine {
    pub fn new(name: String) -> Self {
        let mut data: Vec<Map<String,Value>> = vec![];
        info!("Creating JSON Engine...");
        Self { name, data }
    }
    
    pub fn insert_one(&mut self, new_data: Value) -> String {
        self.data.push(new_data);
        println!("SELF DATA: {:#?}", self.data);
    }

    pub fn insert_many(&mut self) -> Vec<String> {
        todo!();
    }

    pub fn find_one(&self) {
        todo!();
    }

    pub fn find_all(&self) {
        todo!();
    }

    pub fn find_with_filter(&self) {
        todo!();
    }

    pub fn commit(&self) {
        todo!();
    }
    
}





// C bindings
#[no_mangle]
pub extern "C" fn json_engine_new(name: *const c_char) -> *mut JSONEngine {
    let c_str = unsafe { CStr::from_ptr(name) };
    let rust_str = c_str.to_str().unwrap();
    Box::into_raw(Box::new(JSONEngine::new(rust_str.to_string())))
}

#[no_mangle]
pub extern "C" fn json_engine_add_data(engine_ptr: *mut JSONEngine, data: *const c_char) {
    let engine = unsafe { &mut *engine_ptr };
    let c_data_str = unsafe { CStr::from_ptr(data) };
    let rust_data = c_data_str.to_str().unwrap();
    engine.add_data(rust_data);
}

#[no_mangle]
pub extern "C" fn json_engine_get_data(engine_ptr: *mut JSONEngine) -> *mut c_char {
    let engine = unsafe { &*engine_ptr };
    let data = engine.get_data();
    let c_string = CString::new(data).expect("CString::new failed");
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn json_engine_free(engine_ptr: *mut JSONEngine) {
    if engine_ptr.is_null() { return; }
    unsafe { Box::from_raw(engine_ptr) };  // Automatically cleaned up
}

// Free allocated C string
#[no_mangle]
pub extern "C" fn json_engine_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s) };  // Automatically cleaned up
}

//!
//!    Filename: types.rs
//!    Description: Global types for AliceDB
//!    Author: Daniil (0xJanus) Ermolaev
//!

use std::error::Error;

/// Default T or Error Boxed Result.
/// 
/// Examples
/// 
/// ```
/// pub fn some_function() -> BoxedResult<String> { ... }
/// pub fn another_function() -> BoxedResult<i32> { ... }
/// ```
pub type BoxedResult<T> = std::result::Result<T, Box<dyn Error>>;
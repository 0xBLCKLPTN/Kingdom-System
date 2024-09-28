//!
//!    Filename: types.rs
//!    Description: Global types for AliceDB
//!    Author: Daniil (0xJanus) Ermolaev
//!

use std::error::Error;

pub type BoxedResult<T> = std::result::Result<T, Box<dyn Error>>;
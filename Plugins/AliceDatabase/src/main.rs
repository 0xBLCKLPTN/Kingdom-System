//!
//!    Filename: main.rs
//!    Description: Start point of AliceDB plugin.
//!    Author: Daniil (0xJanus) Ermolaev
//!

pub mod adbcore;
use adbcore::types::BoxedResult;

use std::path::PathBuf;
use adbcore::fs_manager::*;

#[tokio::main]
async fn main() -> BoxedResult<()> {
    Ok(())
}

pub mod tests;
use tests::*;

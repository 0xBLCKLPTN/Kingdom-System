/*                          MIT License

Copyright (c) 2024 Daniil Ermolaev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */

pub use std::{ env, fs, fmt };
pub use std::io::{ self, Read, Write, BufRead };
pub use std::path::{ PathBuf, Path };
pub use std::sync::{ Arc, Mutex };

pub use pest_derive::Parser;
pub use pest::Parser;

pub use serde_json::{ json, Value, Result as JsonResult };

pub use log::{ info, error, trace };
pub use simplelog::*;

pub use chrono::Local;

/*
pub mod json_engine;
pub mod log_engine;
pub mod engines;
pub mod grpc_server;
pub mod instance;
pub mod utils;
pub mod command_executor;
pub mod cli;
*/

pub use crate::json_engine::*;
pub use crate::log_engine::*;

pub use crate::engines::*;
pub use crate::grpc_server::*;
pub use crate::instance::*;
pub use crate::utils::*;
pub use crate::command_executor::*;
pub use crate::cli::cli;

pub use tonic::{ transport::Server, Request, Response, Status };

pub use uuid::Uuid;
pub use std::collections::HashMap;


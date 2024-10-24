use crate::json_engine::JSONEngine;

#[derive(Debug, Clone)]
pub enum Engines {
    JSONEngine(JSONEngine),
}
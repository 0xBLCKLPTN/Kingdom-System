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

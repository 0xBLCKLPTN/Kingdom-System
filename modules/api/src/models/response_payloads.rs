use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DefaultResponse<'aa> {
    pub status: &'aa str,
    pub message: &'aa str,
}

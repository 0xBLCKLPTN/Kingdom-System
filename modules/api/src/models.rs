use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub username: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: Token,
    pub refresh_token: Token,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct BasicResponse {
    pub status: String,
    pub message: String,
}

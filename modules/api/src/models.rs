use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: Token,
    pub refresh_token: Token,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
    pub username: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Teacher {
    pub name: String
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims { pub sub: String, pub exp: usize }

#[derive(Serialize, Deserialize)]
pub struct LoginRequest { pub username: String, pub password: String }

#[derive(Serialize, Deserialize)]
pub struct TokenResponse { pub access_token: String, pub token_type: String }

#[derive(Serialize, Deserialize)]
pub struct Task { pub id: i32, pub description: String, pub done: bool }

use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::error;

#[derive(Debug)]
pub struct LoginError {
    pub message: String,
}

impl LoginError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl error::Error for LoginError {}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
    supported_user_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: AccessToken,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    token: String,
    expiration_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: Error,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    code: i32,
    developer_message: String,
    user_message: String,
}

pub async fn login(username: String, password: String) -> Result<String, LoginError> {
    let client = reqwest::Client::new();

    let request_body = LoginRequest {
        username,
        password,
        supported_user_types: vec![String::from("child")],
    };

    let response = client
        .post("https://www.easistent.com/m/login")
        .header("X-App-Name", "child")
        .header("X-Client-Version", "11101")
        .header("X-Client-Platform", "android")
        .json::<LoginRequest>(&request_body)
        .send()
        .await
        .map_err(|e| LoginError::new(e.to_string().as_ref()))?;

    if !response.status().is_success() {
        let response_body = response
            .json::<ErrorResponse>()
            .await
            .map_err(|e| LoginError::new(e.to_string().as_ref()))?;

        return Err(LoginError::new(&response_body.error.user_message));
    }

    let response_body = response
        .json::<LoginResponse>()
        .await
        .map_err(|e| LoginError::new(e.to_string().as_ref()))?;

    Ok(response_body.access_token.token)
}

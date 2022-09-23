use std::{error, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Timetable {
    pub events: Vec<TimetableEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableEvent {
    pub date: String,
    pub from: String,
    pub to: String,
    pub title: String,
    pub classroom: String,
}

#[derive(Debug)]
pub struct TimetableError {
    pub message: String,
}

impl TimetableError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl error::Error for TimetableError {}

impl fmt::Display for TimetableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn get_timetable(token: &str, from: &str, to: &str) -> Result<Timetable, TimetableError> {
    let client = reqwest::Client::new();

    let timetable = client
        .get(format!(
            "https://www.easistent.com/m/timetable/events?from={from}&to={to}"
        ))
        .bearer_auth(token)
        .header("app", "new_mobile_app_2")
        .header("x-app-name", "child")
        .header("x-client-platform", "web")
        .header("x-client-version", "13")
        .send()
        .await
        .map_err(|_| TimetableError::new("Request failed"))?
        .json::<Timetable>()
        .await
        .map_err(|_| TimetableError::new("Failed to parse json"))?;

    Ok(timetable)
}

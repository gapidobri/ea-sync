use chrono::{DateTime, Utc};

use self::{
    login::{login, LoginError},
    timetable::{get_timetable, Timetable, TimetableError},
};

mod login;
mod timetable;

pub struct EAsistent {
    token: Option<String>,
}

impl EAsistent {
    pub fn new() -> Self {
        Self { token: None }
    }
    pub async fn login(&mut self, username: String, password: String) -> Result<(), LoginError> {
        self.token = Some(login(username, password).await?);
        Ok(())
    }
    pub async fn get_timetable(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Timetable, TimetableError> {
        if let Some(token) = &self.token {
            get_timetable(
                token.as_str(),
                from.date_naive().format("%Y-%m-%d").to_string().as_str(),
                to.date_naive().format("%Y-%m-%d").to_string().as_str(),
            )
            .await
        } else {
            Err(TimetableError::new("Not logged in"))
        }
    }
}

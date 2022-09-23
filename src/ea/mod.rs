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
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), LoginError> {
        self.token = Some(login(username, password).await?);
        Ok(())
    }
    pub async fn get_timetable(&self, from: &str, to: &str) -> Result<Timetable, TimetableError> {
        if let Some(token) = &self.token {
            get_timetable(token.as_str(), from, to).await
        } else {
            Err(TimetableError::new("Not logged in"))
        }
    }
}

use core::fmt;
use std::{collections::HashMap, error};
use tl::VDom;

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

pub async fn login(username: &str, password: &str) -> Result<String, LoginError> {
    let client = reqwest::Client::new();

    let mut params = HashMap::<&str, &str>::new();
    params.insert("uporabnik", username);
    params.insert("geslo", password);

    let login_res = client
        .post("https://www.easistent.com/p/ajax_prijava")
        .form(&params)
        .send()
        .await
        .map_err(|_| LoginError::new("Login request failed"))?;

    let cookie = login_res
        .headers()
        .get("set-cookie")
        .ok_or_else(|| LoginError::new("Cookie is missing in response"))?
        .to_str()
        .map_err(|_| LoginError::new("Failed to parse cookie"))?;

    let html = client
        .get("https://www.easistent.com/webapp")
        .header("cookie", cookie)
        .send()
        .await
        .map_err(|_| LoginError::new("Token request failed"))?
        .text()
        .await
        .map_err(|_| LoginError::new("Retrieving body failed"))?;

    let dom = tl::parse(html.as_str(), tl::ParserOptions::default())
        .map_err(|_| LoginError::new("Failed to parse DOM"))?;

    let token = extract_token(&dom).ok_or_else(|| LoginError::new("Failed to extract token"))?;

    Ok(token)
}

fn extract_token(dom: &VDom) -> Option<String> {
    Some(
        dom.query_selector("meta[name='access-token']")?
            .next()?
            .get(dom.parser())?
            .as_tag()?
            .attributes()
            .get("content")??
            .as_utf8_str()
            .to_string()
            .split(" ")
            .last()?
            .to_string(),
    )
}

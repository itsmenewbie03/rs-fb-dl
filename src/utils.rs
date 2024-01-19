use dotenv::dotenv;
use reqwest;
use std::{collections::HashMap, fs};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Media {
    url: String,
    quality: String,
    extension: String,
    size: u64,
    formattedSize: String,
    videoAvailable: bool,
    audioAvailable: bool,
    chunked: bool,
    cached: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Item {
    url: String,
    title: String,
    thumbnail: String,
    duration: Option<u32>,
    source: String,
    medias: Vec<Media>,
    sid: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorResp {
    error: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum ReponseFormatResult {
    Item(Item),
    ErrorResp(ErrorResp),
}
pub async fn get_page_source() -> String {
    dotenv().ok();
    let url = std::env::var("URL").expect("NO URL FOUND IN THE CURRENT ENV");
    let res = reqwest::get(url).await;
    let text = match res {
        Ok(res) => res.text().await,
        Err(err) => Err(err),
    };
    match text {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

pub fn extract_token(html: &str) -> String {
    let token = html.split("name=\"token\" value=\"").collect::<Vec<&str>>()[1]
        .split("\"")
        .collect::<Vec<&str>>()[0]
        .to_string();
    token
}

pub fn load_token() -> String {
    let token = fs::read_to_string(".token");
    match token {
        Ok(token) => token.trim().to_string(),
        Err(_) => "".to_string(),
    }
}

pub fn save_token(token: &str) {
    fs::write(".token", token).expect("UNABLE TO SAVE TOKEN");
}

pub async fn download_video(fb_url: &str, token: &str) -> String {
    dotenv().ok();
    let url = std::env::var("DL_ENDPOINT").expect("NO DL_ENDPOINT FOUND IN THE CURRENT ENV");
    let cli = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("token", token);
    body.insert("url", fb_url);
    let res = cli.post(url).json(&body).send().await;
    let text = match res {
        Ok(res) => res.text().await,
        Err(err) => Err(err),
    };
    match text {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

pub fn format_response(resp: &str) -> ReponseFormatResult {
    println!("resp: {}", resp);
    let item: ReponseFormatResult = serde_json::from_str(resp).unwrap();
    item
}

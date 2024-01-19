use dotenv::dotenv;
use reqwest;
use std::{collections::HashMap, fs};

pub async fn get_page_source() -> String {
    dotenv().ok();
    let url = std::env::var("URL").expect("NO URL FOUND IN THE CURRENT ENV");
    let res = reqwest::get(url).await.unwrap().text().await;
    match res {
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
        Ok(token) => token,
        Err(_) => "".to_string(),
    }
}

pub fn save_token(token: &str) {
    fs::write(".token", token).expect("UNABLE TO SAVE TOKEN");
}

pub async fn download_video(fb_url: &str, token: &str) {
    dotenv().ok();
    let url = std::env::var("DL_ENDPOINT").expect("NO DL_ENDPOINT FOUND IN THE CURRENT ENV");
    let cli = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("token", token);
    body.insert("url", fb_url);
    let res = cli.post(url).json(&body).send().await;
    match res {
        Ok(res) => println!("{}", res.text().await.unwrap()),
        Err(err) => println!("{}", err.to_string()),
    }
}

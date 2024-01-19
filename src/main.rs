mod utils;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqBody {
    url: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}

#[post("/fbdl")]
async fn fb_dl(req_body: web::Json<ReqBody>) -> Result<impl Responder> {
    let mut token = utils::load_token();
    if token == "" {
        let page_source = utils::get_page_source().await;
        let new_token = utils::extract_token(&page_source);
        utils::save_token(&new_token);
        token = new_token;
    }
    println!("trying to download: {}", &req_body.url);
    let dl_resp = utils::download_video(&req_body.url, &token).await;
    let resp = utils::format_response(&dl_resp);
    Ok(web::Json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(fb_dl))
        .bind(("127.0.0.1", 6969))?
        .run()
        .await
}

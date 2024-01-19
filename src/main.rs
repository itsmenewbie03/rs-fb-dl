mod utils;

#[tokio::main]
async fn main() {
    let mut token = utils::load_token();
    if token == "" {
        println!("Token not found, fetching...");
        let page_source = utils::get_page_source().await;
        let new_token = utils::extract_token(&page_source);
        utils::save_token(&new_token);
        token = new_token;
    }
    println!("Token found: {}", token);
    utils::download_video("https://fb.watch/pG68jU6gWn/", &token).await;
}

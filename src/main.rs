mod qq_robot_api;
mod request_client;
mod rss;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();

    // qq_robot_api::websocket_api::init_global_wss_stream().await.unwrap();
    
    // let listen = qq_robot_api::websocket_api::listen_wss();
    // let _res1 = tokio::join!(listen);

    let _ = rss::rss::get_rss("https://mikanani.me/RSS/MyBangumi?token=dgnDIkWrOX0TOC5lTJRmgg%3d%3d".to_string()).await;
    Ok(())
}


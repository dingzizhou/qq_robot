mod qq_robot_api;
mod request_client;
mod rss;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();
    config::init();
    // let _ = qq_robot_api::websocket_api::init_global_wss_stream().await?;
    
    // let listen = qq_robot_api::websocket_api::listen_wss();

    let _ = rss::rss::get_rss().await;
    // let _res1 = tokio::join!(listen);
    Ok(())
}


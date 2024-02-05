mod qq_robot_api;
mod request_client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();
    
    // let websocket1 = qq_robot_api::websocket_api::connect_to_wss();
    qq_robot_api::websocket_api::init_global_wss_stream().await.unwrap();
    // let websocket2 = qq_robot_api::websocket_api::connect_to_wss();
    // let _ = qq_robot_api::websocket_api::WSS_CLOSE_FLAG.set(true);
    
    let listen = qq_robot_api::websocket_api::listen_wss();
    let _res1 = tokio::join!(listen);
    Ok(())
}


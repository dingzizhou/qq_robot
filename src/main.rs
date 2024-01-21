mod qq_robot_api;
mod request_client;

use std::option::Option;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();
    println!("123");

    let websocket1 = qq_robot_api::websocket_api::connect_to_wss();
    // let websocket2 = qq_robot_api::websocket_api::connect_to_wss();
    println!("456");
    // let _ = qq_robot_api::websocket_api::WSS_CLOSE_FLAG.set(true);
    let res1 = tokio::join!(websocket1);
    Ok(())
}


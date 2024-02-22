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

    // let res = qq_robot_api::guilds::get_channels_list().await;
    // match res {
    //     Ok(_) => println!("ok"),
    //     Err(value) => println!("{:?}",value),
    // }
    let _ = rss::rss::run().await;
    // let _res1 = tokio::join!(listen);
    Ok(())
}


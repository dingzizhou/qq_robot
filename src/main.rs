use futures_util::join;

mod qq_robot_api;
mod request_client;
mod rss;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();
    config::init();
    let _ = qq_robot_api::websocket_api::init_global_wss_stream().await?;
    
    // let listen = qq_robot_api::websocket_api::listen_wss();
    let res1 = tokio::spawn( async {
        let _ = qq_robot_api::websocket_api::listen_wss().await;
    });
    // let res = qq_robot_api::guilds::get_channels_list().await;
    // match res {
    //     Ok(_) => println!("ok"),
    //     Err(value) => println!("{:?}",value),
    // }
    // let res2 = tokio::spawn(async {
    //     rss::rss::run().await;
    // });
    // let rss = rss::rss::run().await;
    let _res1 = tokio::join!(res1);
    // let _res2 = tokio::join!(res2);
    Ok(())
}


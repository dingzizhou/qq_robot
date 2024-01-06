mod qq_robot_api;
mod request_client;

use std::option::Option;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();
    println!("123");

    // let res = qq_robot_api::app_access_token::get_global_access_token().await?;
    // println!("{:?}",res);
    // let _ = qq_robot_api::guilds::get_channels_list().await?;

    // let _ = qq_robot_api::message::send_channel_message(Option::Some("test_send_null_param".to_string()),Option::None).await?;
    Ok(())
}


use std::collections::HashMap;

use crate::config;
use crate::request_client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageEmbed {
    
}

#[derive(Serialize)]
pub struct MessageArk {

}

#[derive(Serialize)]
pub struct MessageReference {

}

#[derive(Serialize)]
pub struct MessageMarkdown {

}

pub async fn send_channel_message(content:Option<String>,embed:Option<MessageEmbed>,ark:Option<MessageArk>,message_reference:Option<MessageReference>
                                    ,image:Option<String>,msg_id:Option<String>,event_id:Option<String>,markdown:Option<MessageMarkdown>) -> Result<(), Box<dyn std::error::Error>>{
    let mut request_data = HashMap::new();
    // request_data.insert("content", content.unwrap());
    match content {
        Some(t) => request_data.insert("content", t),
        None => None,
    };
    match embed {
        Some(t) => request_data.insert("embed", serde_json::to_string(&t)?),
        None => None,
    };
    match ark {
        Some(t) => request_data.insert("ark", serde_json::to_string(&t)?),
        None => None,
    };
    match message_reference {
        Some(t) => request_data.insert("message_reference", serde_json::to_string(&t)?),
        None => None,
    };
    match image {
        Some(t) => request_data.insert("image", t),
        None => None,
    };
    match msg_id {
        Some(t) => request_data.insert("msg_id", t),
        None => None,
    };
    match event_id {
        Some(t) => request_data.insert("event_id", t),
        None => None,
    };
    match markdown {
        Some(t) => request_data.insert("markdown", serde_json::to_string(&t)?),
        None => None,
    };
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    println!("{:?}",token);
    let res = request_client::REQUEST_CLIENT.get().unwrap().post("https://api.sgroup.qq.com/channels/634792030/messages")
                                                                    .header("Authorization",token)
                                                                    .header("X-Union-Appid", config::QQROBOT_APPID.get().unwrap().as_str())
                                                                    .json(&request_data)
                                                                    .send()
                                                                    .await?
                                                                    .text()
                                                                    .await?;
    println!("{:?}",res);
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_send_channel_message() {
//         send_channel_message("test_send_channel_message".to_string(), "image".to_string()).await;
//     }
// }
use std::collections::HashMap;

use reqwest::Url;
use serde_json::Map;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use std::sync::OnceLock;

use crate::request_client;

static WSS_CLOSE_FLAG:OnceLock<bool> = OnceLock::new();

#[derive(Deserialize,Debug)]
struct WssUrl {
    url:String,
}

#[derive(Deserialize,Debug)]
struct Identify {
    token:String,
    intents:i32,
    shard:[u8;2],
    properties:HashMap<String,String>,
}

#[derive(Deserialize,Debug,Serialize)]
struct Payload {
    op:u8,
    d:serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    s:Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    t:Option<String>,
}

enum IntentsEnum {
    GUILDS = 1 << 0,
    GUILD_MEMBERS = 1 << 1,
    GUILD_MESSAGES = 1 << 9,
    GUILD_MESSAGE_REACTIONS = 1 << 10,
    DIRECT_MESSAGE = 1 << 12,
    INTERACTION = 1 << 26,
    MESSAGE_AUDIT = 1 << 27,
    FORUMS_EVENT = 1 << 28,
    AUDIO_ACTION = 1 << 29,
    PUBLIC_GUILD_MESSAGES  = 1 << 30,
}

#[derive(Deserialize,Debug)]
enum OpcodeEnum {
    Dispatch = 0,// 服务端进行消息推送
    Heartbeat = 1,// 客户端或服务端发送心跳
    Identify = 2,// 客户端发送鉴权
    Resume = 6,// 客户端恢复连接
    Reconnect = 7,// 服务端通知客户端重新连接
    InvalidSession = 9,// 当identify或resume的时候，如果参数有错，服务端会返回该消息
    Hello = 10,// 当客户端与网关建立ws连接之后，网关下发的第一条消息
    HeartbeatACK = 11,// 当发送心跳成功之后，就会收到该消息
    HTTPCallbackACK = 12,// 仅用于 http 回调模式的回包，代表机器人收到了平台推送的数据
}

async fn get_wss_url() -> Result<String, Box<dyn std::error::Error>> {
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://sandbox.api.sgroup.qq.com/gateway")
                                                                .header("Authorization",token)
                                                                .header("X-Union-Appid", "102079646")
                                                               .send()
                                                               .await?
                                                               .json::<WssUrl>()
                                                               .await?;
    Ok(res.url.to_string())
}

pub async fn connect_to_wss() -> Result<(), Box<dyn std::error::Error>>{
    let _ = WSS_CLOSE_FLAG.set(false);
    let wss_url = get_wss_url().await?;
    println!("wss_url:{}",wss_url);
    let (mut ws_stream,_) = connect_async(&wss_url).await.expect("Fail to connect");
    loop {
        println!("{:?}",WSS_CLOSE_FLAG.get().unwrap());
        if *WSS_CLOSE_FLAG.get().unwrap() {
            break;
        }
        let res = ws_stream.next().await.expect("Cant fetch case count").unwrap();
        println!("res = {:?}",&res.clone().to_string());
        let res_object:Payload = serde_json::from_str(&res.clone().to_string()).unwrap();
        println!("res_object = {:?}",res_object);
    }
    
    // let identify = Identify {
    //     token: crate::qq_robot_api::app_access_token::get_global_access_token().await?,
    //     intents: IntentsEnum::PUBLIC_GUILD_MESSAGES as i32,
    //     shard: [0,1],
    //     properties: HashMap::new(),
    // };
    // println!("{:?}",identify);
    println!("close ws_stream");
    ws_stream.close(None).await?;
    Ok(())
}
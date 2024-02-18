use std::{collections::HashMap, thread};
use std::sync::{Arc, OnceLock};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{
    lock::Mutex,
    StreamExt,
    sink::{Sink, SinkExt,},
    stream::{FusedStream, Stream},
};
// use futures_util::{stream::FusedStream, SinkExt, StreamExt, Sink};
use serde_json::json;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{config, request_client};


#[derive(Deserialize,Debug)]
struct WssUrl {
    url:String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Identify {
    token:String,
    intents:i32,
    shard:[u8;2],
    properties:HashMap<String,String>,
}

#[derive(Serialize,Deserialize,Debug)]
struct ReadyEvent {
    version:u8,
    session_id:String,
    user: ReadyEventUser,
    shard:[u8;2],
}

#[derive(Serialize,Deserialize,Debug)]
struct ReadyEventUser {
    id:String,
    username:String,
    bot:bool,
}

#[derive(Deserialize,Debug,Serialize,Clone)]
struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    op:Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    d:Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s:Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    t:Option<String>,
}

enum IntentsEnum {
    GUILDS = 1 << 0,
    GuildMembers = 1 << 1,
    GuildMessages = 1 << 9,
    GuildMessageReactions = 1 << 10,
    DirectMessage = 1 << 12,
    INTERACTION = 1 << 26,
    MessageAudit = 1 << 27,
    ForumsEvent = 1 << 28,
    AudioAction = 1 << 29,
    PublicGuildMessages  = 1 << 30,
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

pub struct WssStruct {
    // wss_stream:WebSocketStream<MaybeTlsStream<TcpStream>>,
    read_stream:Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    write_stream:Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>,Message>>>,
    ack:Arc<Mutex<Option<u32>>>,
    heartbeat_interval:Arc<Mutex<Option<HashMap<String,u64>>>>,
    ready_event:Arc<Mutex<Option<Payload>>>,
}

pub static GLOBAL_WSS_STRUCT:OnceLock<WssStruct> = OnceLock::new();

pub async fn init_global_wss_stream() -> Result<bool, Box<dyn std::error::Error>> {
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://sandbox.api.sgroup.qq.com/gateway")
                                                                .header("Authorization",token)
                                                                .header("X-Union-Appid", config::QQROBOT_APPID.get().unwrap().as_str())
                                                                .send()
                                                                .await?
                                                                .json::<WssUrl>()
                                                                .await?;
    let ( wss_stream , _ ) = connect_async(res.url).await.unwrap();
    let (write_stream, read_stream) = wss_stream.split();
    // let res1 = write.send(Message::Text(serde_json::to_string("1").unwrap()));
    // let res2 = read.next();
    // let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    
    let wss_struct = WssStruct{
        read_stream:Arc::new(Mutex::new(read_stream)),
        write_stream:Arc::new(Mutex::new(write_stream)),
        ack: Arc::new(Mutex::new(Some(0))),
        heartbeat_interval: Arc::new(Mutex::new(None)),
        ready_event: Arc::new(Mutex::new(None)),
    };
    let _ = GLOBAL_WSS_STRUCT.set(wss_struct);
    // let send_payload = Payload{
    //     op: todo!(),
    //     d: todo!(),
    //     s: todo!(),
    //     t: todo!(),
    // };
    // write_stream.send(item)
    Ok(true)
}

// pub async fn listen_wss() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }

pub async fn listen_wss() -> Result<(), Box<dyn std::error::Error>>{
    let wss_struct = GLOBAL_WSS_STRUCT.get().unwrap();
    loop {
        let res = wss_struct.read_stream.lock().await.next().await.expect("Cant fetch case count").unwrap();
        println!("res = {:?}",res);
        let res_object:Payload = serde_json::from_str(&res.to_string()).unwrap_or_else(|err| {
            println!("err = {:?}", err);
            let err_payload = Payload {
                op: Some(6),
                d: None,
                s: None,
                t: None,
            };
            err_payload.clone()
        });
        if res_object.s.is_some() {
            *wss_struct.ack.lock().await = res_object.s;
        }
        if res_object.op.is_none() { continue };
        match res_object.op.unwrap() {
            // Dispatch 服务端进行消息推送
            0 => {
                
            },
            // Heartbeat 客户端或服务端发送心跳
            1 => {
            },
            // Reconnect 	服务端通知客户端重新连接
            7 => {
            },
            // Invalid Session
            // 当 identify 或 resume 的时候，如果参数有错，服务端会返回该消息
            9 => {
                println!("InvalidSession");
            },
            // Hello
            // 当客户端与网关建立 ws 连接之后，网关下发的第一条消息
            10 => {
                *wss_struct.heartbeat_interval.lock().await = serde_json::from_value(res_object.d.into()).unwrap();
                let identify = Identify {
                    token: crate::qq_robot_api::app_access_token::get_global_access_token().await?,
                    intents: 0 | IntentsEnum::PublicGuildMessages as i32,
                    shard: [0,1],
                    properties: HashMap::new(),
                };
                let req_payload = Payload {
                    op:Some(2),
                    d:Some(json!(&identify)),
                    s:None,
                    t:None,
                };
                {
                    let _ = wss_struct.write_stream.lock().await.send(Message::Text(serde_json::to_string(&req_payload).unwrap())).await;
                }
                println!("send identify");
                let ready_event = match wss_struct.read_stream.lock().await.next().await.expect("Cant fetch case count") {
                    Ok(value) => {
                        // println!("value = {:?}",value);
                        serde_json::from_str(&value.to_string()).unwrap()
                    },
                    Err(_) => {
                        // println!("err = {:?}",err);
                        Some(Payload {
                            op:None,
                            d:None,
                            s:None,
                            t:None,
                        })
                    },
                };
                *wss_struct.ready_event.lock().await = ready_event;
                let ack_payload = Payload {
                    op:Some(1),
                    d:None,
                    s:None,
                    t:None,
                };
                {
                    let _ = wss_struct.write_stream.lock().await.send(Message::Text(serde_json::to_string(&ack_payload).unwrap())).await;
                }
                tokio::spawn(async move{
                    send_heartbeat().await;
                });
            },
            // Heartbeat ACK
            // 当发送心跳成功之后，就会收到该消息
            11 => {

            },
            // HTTP Callback ACK
            // 仅用于 http 回调模式的回包，代表机器人收到了平台推送的数据
            12 => {

            }
            _ => {

            }
        }
        
    }
    Ok(())
}

async fn send_heartbeat(){
    // println!("start send_heartbeat function");
    let wss_struct = GLOBAL_WSS_STRUCT.get().unwrap();
    let heartbeat_interval;
    {
        heartbeat_interval = wss_struct.heartbeat_interval.lock().await.clone().unwrap();
    }
    let ms = heartbeat_interval.get("heartbeat_interval").unwrap();
    // println!("ms = {ms}");
    let duration = std::time::Duration::from_millis(*ms-200);
    loop {
        thread::sleep(duration);
        // println!("send heartbeat");
        // println!("get d");
        let d;
        {
            d = (*wss_struct.ack.lock().await).unwrap();
        }
        // println!("d = {d}");
        let ack_payload = Payload {
            op:Some(1),
            d:Some(d.into()),
            s:None,
            t:None,
        };
        // println!("heartbeat ack_payload = {:?}",ack_payload);
        {
            let _send_res = wss_struct.write_stream.lock().await.send(Message::Text(serde_json::to_string(&ack_payload).unwrap())).await;
        }
        println!("send heartbeat success ack_payload = {:?}",ack_payload);
    }
}
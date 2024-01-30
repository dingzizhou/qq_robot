use std::collections::HashMap;

use futures_util::{stream::FusedStream, SinkExt, StreamExt, TryFutureExt};
use serde_json::json;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use std::sync::OnceLock;

use crate::request_client;

pub static WSS_CLOSE_FLAG:OnceLock<bool> = OnceLock::new();
static ACK:OnceLock<u32> = OnceLock::new();

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
    wss_stream:WebSocketStream<MaybeTlsStream<TcpStream>>,
    ack:Option<u32>,
    heartbeat_interval:Option<HashMap<String,u32>>,
    ready_event:Option<Payload>,
}

pub async fn get_wss_struct() -> Result<WssStruct, Box<dyn std::error::Error>> {
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://sandbox.api.sgroup.qq.com/gateway")
                                                                .header("Authorization",token)
                                                                .header("X-Union-Appid", "102079646")
                                                                .send()
                                                                .await?
                                                                .json::<WssUrl>()
                                                                .await?;
    let ( wss_stream , _ ) = connect_async(res.url).await.unwrap();
    let wss_struct = WssStruct{
        wss_stream,
        ack: Some(0),
        heartbeat_interval: None,
        ready_event: None,
    };
    Ok(wss_struct)
}

impl WssStruct {
    
    pub async fn listen_wss(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        loop {
            if self.wss_stream.is_terminated(){
                break;
            }
            // println!("{:?}",WSS_CLOSE_FLAG.get().unwrap());
            // if self.wss_close_flag {
            //     break;
            // }
            let res = self.wss_stream.next().await.expect("Cant fetch case count").unwrap_or_else(|err| {
                println!("err = {:?}", err);
                Message::Text("err".to_string())
            });
            println!("res = {:?}",res);
            let res_object:Payload = serde_json::from_str(&res.to_string()).unwrap_or_else(|err| {
                println!("err = {:?}", err);
                let err_payload = Payload {
                    op: None,
                    d: None,
                    s: None,
                    t: None,
                };
                err_payload.clone()
            });
            // println!("res_object = {:?}",res_object);
            self.ack = Some(res_object.s.unwrap());
            match res_object.op.unwrap() {
                // Dispatch 服务端进行消息推送
                0 => {
                },
                1 => {
                },
                2 => {
                },
                // Resume 客户端回复链接
                6 => {
                },
                7 => {
                },
                9 => {
                    println!("InvalidSession");
                },
                // Hello 当客户端与网关建立 ws 连接之后，网关下发的第一条消息
                10 => {
                    self.heartbeat_interval = serde_json::from_value(res_object.d.into()).unwrap();
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
                    let _ = self.wss_stream.send(Message::Text(serde_json::to_string(&req_payload).unwrap())).await;
                    println!("send identify");
                    self.ready_event = match self.wss_stream.next().await.expect("Cant fetch case count") {
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
                    let ack_payload = Payload {
                        op:Some(1),
                        d:None,
                        s:None,
                        t:None,
                    };
                    let _ = self.wss_stream.send(Message::Text(serde_json::to_string(&ack_payload).unwrap())).await;
                    // let ack_res = ws_stream.next().await.expect("Cant fetch case count").unwrap();
                    // ack = serde_json::from_str(&ack_res.to_string()).unwrap();
                    self.send_heartbeat();
                },
                _ => {
    
                }
            }
            
        }
        
        // println!("close ws_stream");
        // ws_stream.close(None).await?;
        Ok(())
    }
    
    async fn send_heartbeat(&self){
        let ack_payload = Payload {
            op:Some(self.ack.unwrap()),
            d:None,
            s:None,
            t:None,
        };
    }

}
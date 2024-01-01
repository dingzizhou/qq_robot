use std::collections::HashMap;

use crate::request_client;
use serde::Deserialize;
use std::sync::OnceLock;
use chrono::{Utc, DateTime};

#[derive(Deserialize)]
struct QQtoken {
    access_token:String,
    expires_in:String
}

pub struct AccessToken {
    pub token:String,
    update_time:DateTime<Utc>
}

static GLOBAL_ACCESS_TOKEN:OnceLock<AccessToken> = OnceLock::new();

async fn get_app_access_token() -> Result<QQtoken, Box<dyn std::error::Error>>{
    // println!("get_app_access_token");
    let mut data_map = HashMap::new();
    data_map.insert("appId","102079646");
    data_map.insert("clientSecret", "Us1yhCTWLxNZYIp7");
    let res = request_client::REQUEST_CLIENT.get().unwrap().post("https://bots.qq.com/app/getAppAccessToken")
                                                                    .json(&data_map)
                                                                    .send()
                                                                    .await?
                                                                    .json::<QQtoken>()
                                                                    .await?;
    
    // println!("{:?},{:?}",res.access_token,res.expires_in);
    Ok(res)
}

pub async fn get_global_access_token() -> Result<String, Box<dyn std::error::Error>>{
    let token = GLOBAL_ACCESS_TOKEN.get();
    let now_time = Utc::now();
    match token{
        Some(t)=>{
            // println!("更新前: {:?}",t.update_time);
            if t.update_time <= now_time {
                let res = get_app_access_token().await?;
                let _ = GLOBAL_ACCESS_TOKEN.set(
                    AccessToken{
                        token:res.access_token.to_string(),
                        update_time:now_time + chrono::Duration::seconds(res.expires_in.parse::<i64>().unwrap())
                });
            }
            // println!("更新后: {:?}",t.update_time);
            Ok(t.token.clone())
        },
        None=>{
            let res = get_app_access_token().await?;
            let _ = GLOBAL_ACCESS_TOKEN.set(
                AccessToken{
                    token:res.access_token.to_string(),
                    update_time:now_time + chrono::Duration::seconds(res.expires_in.parse::<i64>().unwrap())
            });
            Ok(res.access_token.to_string())
        }
    }
}
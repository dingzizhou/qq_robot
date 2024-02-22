
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use crate::{config, request_client};

#[derive(Deserialize,Debug)]
struct Guild {
    id:String,
    name:String,
    icon:String,
    owner_id:String,
    owner:bool,
    member_count:i32,
    max_members:i32,
    description:String,
    joined_at:String
}

#[derive(Serialize,Deserialize,Debug)]
struct Channel{
    id:Option<String>,
    guild_id:Option<String>,
    name:Option<String>,
    r#type:Option<i16>,
    sub_type:Option<i16>,
    position:Option<i16>,
    parent_id:Option<String>,
    owner_id:Option<String>,
    private_type:Option<i16>,
    speak_permission:Option<i16>,
    application_id:Option<String>,
    permissions:Option<String>,
}

static GLOBAL_GUILD_LIST:OnceLock<Vec<Guild>> = OnceLock::new();
static GLOBAL_CHANNEL_LIST:OnceLock<Vec<Channel>> = OnceLock::new();

async fn get_guilds_list() -> Result<Vec<Guild>, Box<dyn std::error::Error>>{
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://api.sgroup.qq.com/users/@me/guilds")
                                                                .header("Authorization",token)
                                                                .header("X-Union-Appid", config::QQROBOT_APPID.get().unwrap().as_str())
                                                                .send()
                                                                .await?
                                                                .json::<Vec<Guild>>()
                                                                .await?;
    // let _ = GLOBAL_GUILD_LIST.set(res);
    // println!("{:?}",GLOBAL_GUILD_LIST.get().unwrap().get(0));
    Ok(res)
}



fn get_guild_id() -> String {
    // GLOBAL_GUILD_LIST.get().unwrap().get(0)
    "7008736632399750680".to_string()
}

pub async fn get_channels_list() -> Result<(), Box<dyn std::error::Error>>{
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    println!("{:?}",token);
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://api.sgroup.qq.com/guilds/".to_string() + &get_guild_id() + "/channels")
                                                                    .header("Authorization",token)
                                                                    .header("X-Union-Appid", config::QQROBOT_APPID.get().unwrap().as_str())
                                                                    .send()
                                                                    .await?
                                                                    .json::<Vec<Channel>>()
                                                                    .await?;
    Ok(())
}
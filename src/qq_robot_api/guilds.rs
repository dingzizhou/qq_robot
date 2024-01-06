
use serde::Deserialize;
use std::sync::OnceLock;
use crate::request_client;

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

#[derive(Deserialize,Debug)]
struct Channel{
    id:String,
    guild_id:String,
    name:String,
    r#type:i16,
    sub_type:i16,
    position:i16,
    parent_id:String,
    owner_id:String
}

static GLOBAL_GUILD_LIST:OnceLock<Vec<Guild>> = OnceLock::new();

pub async fn get_guilds_list() -> Result<(), Box<dyn std::error::Error>>{
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://api.sgroup.qq.com/users/@me/guilds")
                                                                .header("Authorization",token)
                                                                .header("X-Union-Appid", "102079646")
                                                                .send()
                                                                .await?
                                                                .json::<Vec<Guild>>()
                                                                .await?;
    let _ = GLOBAL_GUILD_LIST.set(res);
    println!("{:?}",GLOBAL_GUILD_LIST.get().unwrap().get(0));
    Ok(())
}

fn get_guild_id() -> String {
    "7008736632399750680".to_string()
}

pub async fn get_channels_list() -> Result<(), Box<dyn std::error::Error>>{
    let token = crate::qq_robot_api::app_access_token::get_global_access_token().await?;
    println!("{:?}",token);
    let res = request_client::REQUEST_CLIENT.get().unwrap().get("https://api.sgroup.qq.com/guilds/".to_string() + &get_guild_id() + "/channels")
                                                                    .header("Authorization",token)
                                                                    .header("X-Union-Appid", "102079646")
                                                                    .send()
                                                                    .await?
                                                                    .text()
                                                                    .await?;
    println!("{:?}",res);
    Ok(())
}
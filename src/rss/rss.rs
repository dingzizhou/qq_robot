use std::{collections::{HashMap, HashSet}, thread};

use rss::{Channel, Guid, Item};

use crate::request_client;
use crate::qq_robot_api;

struct RssItem {
    title:String,
    torrent:String,
    magnet:String,
}

pub async fn get_rss() -> Result<Vec<Item>, Box<dyn std::error::Error>>{

    let file = std::fs::File::open("MyBangumi.rss").unwrap();
    let channel = Channel::read_from(std::io::BufReader::new(file));
    // let res = request_client::REQUEST_CLIENT.get().unwrap()
    //                                                         .get(url)
    //                                                         .send()
    //                                                         .await?
    //                                                         .bytes()
    //                                                         .await?;
    // let channel = rss::Channel::read_from(&res[..]);
    // println!("channel = {:?}",channel.unwrap());
    let item = &channel.unwrap().items;
    for i in item{
        println!("{:?}",i);
    }
    Ok(item.to_vec())
}

pub async fn download_item() {

}

pub async fn run() {
    let mut item_list:HashSet<String> = HashSet::new();
    loop {
        let items = get_rss().await.unwrap();
        for item in items {
            if !item_list.get(item.guid().unwrap().value()).is_some() {
                item_list.insert(item.guid().unwrap().value().to_string());
                // println!("{:?}",item);
                let link = item.enclosure().unwrap().url().rsplit('/');
                println!("link = {:?}",link);
                // let content = item.title.unwrap() + "\n" + "08fd03a35d6aad5d6df9314d0a73632c4bc4e621";
                // let _ = qq_robot_api::message::send_channel_message(Some(content), None, None, None, None, None, None, None).await;
                thread::sleep(std::time::Duration::from_millis(1000*10));
            }
        }
        
        // 每隔30分钟获取一次更新
        let duration = std::time::Duration::from_millis(1000*60*30);
        thread::sleep(duration);
    }
}
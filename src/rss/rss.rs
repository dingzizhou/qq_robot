use rss::{Channel, Item};

use crate::request_client;

pub async fn get_rss() -> Result<Vec<Item>, Box<dyn std::error::Error>>{

    let file = std::fs::File::open("MyBangumi.rss").unwrap();
    println!("file = {:?}",file);
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
        println!("{:?}",i.title);
    }
    Ok(item.to_vec())
}

pub async fn download_item() {

}

pub async fn run() {
    loop {
        let items = get_rss().await.unwrap();
        let duration = std::time::Duration::from_millis(1000*60*60*24);
    }
}
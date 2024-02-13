use rss::Channel;

use crate::request_client;



pub async fn get_rss(url:String) -> Result<(), Box<dyn std::error::Error>>{

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
    let item = &channel.unwrap().items[0];
    println!("{:?}",item);
    Ok(())
}
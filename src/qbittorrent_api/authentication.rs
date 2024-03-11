use std::collections::HashMap;

use crate::{request_client,config};

pub async fn login(username:String,password:String) -> Result<(), Box<dyn std::error::Error>>{
    let client = request_client::REQUEST_CLIENT.get().unwrap();
    let mut data_map = HashMap::new();
    data_map.insert("username","123");
    // data_map.insert("password",&password);
    println!("data_map = {:?}",data_map);
    let url = "http://".to_owned() + &config::QBITTORRENT_URL.get().unwrap().clone() + "/api/v2/auth/login?username=admin&password=adminadmin";
    println!("{}", url);
    let res = client.get(url)
                            .header("Referer", "http://localhost:9000")
                            // .json(&data_map);
                            .send()
                            .await?;

    println!("{:?}",res);
    Ok(())
}

#[tokio::test]
async fn login_test(){
    request_client::init();
    config::init();
    let res = login("admin".to_string(), "adminadmin".to_string()).await;
    println!("{:?}",res);
}

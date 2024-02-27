use std::collections::HashMap;

use crate::{request_client,config};

pub async fn login(username:String,password:String){
    let client = request_client::REQUEST_CLIENT.get().unwrap();
    let mut data_map = HashMap::new();
    data_map.insert("username",&username);
    data_map.insert("password",&password);
    // let res = client.post(config::QBITTORRENT_URL.get().unwrap().clone()+"/api/v2/auth/login");
    println!("{:?}",data_map);
}

#[tokio::test]
async fn login_test(){
    request_client::init();
    login("admin".to_string(), "adminadmin".to_string()).await;
    
}

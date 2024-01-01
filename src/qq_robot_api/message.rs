use std::collections::HashMap;

use crate::request_client;

pub fn send_channel_message(content:String) -> Result<String, Box<dyn std::error::Error>>{
    let mut request_data = HashMap::new();
    request_data.insert("content", content);
    
    Ok("".to_string())
}
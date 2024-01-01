
use std::sync::OnceLock;

use reqwest::Client;

pub static REQUEST_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn init(){
    let client = reqwest::Client::new();
    let _ = REQUEST_CLIENT.set(client);
}
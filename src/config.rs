use std::sync::OnceLock;

pub static RSS_URL:OnceLock<String> = OnceLock::new();
pub static QQROBOT_APPID:OnceLock<String> = OnceLock::new();
pub static QQROBOT_CLIENTSECRET:OnceLock<String> = OnceLock::new();
pub static QBITTORRENT_URL:OnceLock<String> = OnceLock::new();

pub fn init(){
    let _ = RSS_URL.set("https://mikanani.me/RSS/MyBangumi?token=dgnDIkWrOX0TOC5lTJRmgg%3d%3d".to_string());
    let _ = QQROBOT_APPID.set("102079646".to_string());
    let _ = QQROBOT_CLIENTSECRET.set("Us1yhCTWLxNZYIp7".to_string());
    let _ = QBITTORRENT_URL.set("localhost:9000".to_string());
}
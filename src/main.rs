mod qq_robot_api;
mod request_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    request_client::init();
    println!("123");

    let res = qq_robot_api::app_access_token::get_global_access_token().await?;
    println!("{:?}",res);
    Ok(())
}


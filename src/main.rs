mod request;
mod error;
mod data;
mod endpoints;
mod client;

use crate::client::{FishFish, FishFishRequestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut default_client = FishFish::new()?;
    let mut auth_client = FishFish::new_with_token("test")?;

    println!("{:?}", auth_client.create_session(None).await?);

    println!("{:?}", default_client.get_status().await?);
    println!("{:?}", auth_client.get_status().await?);

    Ok(())
}
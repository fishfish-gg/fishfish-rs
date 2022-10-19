pub(crate) mod client;
mod default_client;
mod auth_client;

pub use client::{FishFish, FishFishClient, FishFishRequestClient};
pub use default_client::DefaultFishFishClient;
pub use auth_client::AuthFishFishClient;
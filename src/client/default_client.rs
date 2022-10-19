use reqwest::Client;
use crate::client::{FishFishClient, FishFishRequestClient};
use crate::data::Token;

pub struct DefaultFishFishClient {
    pub(crate) http: Client,
}

impl FishFishRequestClient for DefaultFishFishClient {}

impl FishFishClient for DefaultFishFishClient {
    fn get_token(&self) -> Option<&String> { None }

    fn get_session(&self) -> Option<&Token> { None }

    fn http(&self) -> &Client {
        &self.http
    }
}
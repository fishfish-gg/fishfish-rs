use reqwest::{Body, Method};
use crate::error::FishFishError;
use crate::request::web_request::RawWebRequest;

pub trait Endpoint: RawWebRequest {
    fn method(&self) -> Method;
    fn endpoint(&self) -> String;
    fn parameters(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn body(&self) -> Result<Option<Body>, FishFishError> {
        Ok(None)
    }
}


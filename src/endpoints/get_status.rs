use async_trait::async_trait;
use reqwest::{Method};
use serde::Serialize;
use crate::data::{ApiStatus};
use crate::request::{Endpoint, WebRequest};

#[derive(Debug, Serialize, Clone)]
pub struct GetStatusEndpoint {}

impl Endpoint for GetStatusEndpoint {
    fn method(&self) -> Method { Method::GET }

    fn endpoint(&self) -> String {
        format!("https://api.fishfish.gg/v1/status")
    }
}

#[async_trait(?Send)]
impl WebRequest<ApiStatus> for GetStatusEndpoint {}
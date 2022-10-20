use async_trait::async_trait;
use reqwest::{Body, Method};
use serde::Serialize;
use crate::data::{Domain};
use crate::error::FishFishError;
use crate::request::{Endpoint};
use crate::request::web_request::AuthWebRequest;

#[derive(Debug, Serialize, Clone)]
pub struct DeleteDomainEndpoint {
    pub domain: String,
}

impl Endpoint for DeleteDomainEndpoint {
    fn method(&self) -> Method { Method::DELETE }

    fn endpoint(&self) -> String {
        format!("https://api.fishfish.gg/v1/domains/{}", self.domain.as_str())
    }

    fn body(&self) -> Result<Option<Body>, FishFishError> {
        Ok(None)
    }
}

#[async_trait(?Send)]
impl AuthWebRequest<Domain> for DeleteDomainEndpoint {}
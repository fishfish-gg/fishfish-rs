use async_trait::async_trait;
use reqwest::{Body, Method};
use serde::Serialize;
use crate::data::{Domain, DomainCategory};
use crate::error::FishFishError;
use crate::request::{Endpoint};
use crate::request::web_request::AuthWebRequest;

#[derive(Debug, Serialize, Clone)]
pub struct UpdateDomainEndpoint {
    #[serde(skip)]
    pub domain: String,
    pub description: Option<String>,
    pub category: Option<DomainCategory>,
    pub target: Option<String>,
}

impl Endpoint for UpdateDomainEndpoint {
    fn method(&self) -> Method { Method::PATCH }

    fn endpoint(&self) -> String {
        format!("https://api.fishfish.gg/v1/domains/{}", self.domain.as_str())
    }

    fn body(&self) -> Result<Option<Body>, FishFishError> {
        if self.category.eq(&Some(DomainCategory::Unknown)) {
            // TODO: prevent this
        }

        let json = serde_json::ser::to_string(&self)
            .map_err(|_| FishFishError::SerializationError)?;

        Ok(Some(Body::from(json)))
    }
}

#[async_trait(?Send)]
impl AuthWebRequest<Domain> for UpdateDomainEndpoint {}
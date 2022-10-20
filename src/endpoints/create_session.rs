use async_trait::async_trait;
use reqwest::{Body, Method};
use serde::Serialize;
use crate::data::{Permission, Token};
use crate::error::FishFishError;
use crate::request::{Endpoint};
use crate::request::web_request::AuthWebRequest;

#[derive(Debug, Serialize, Clone, Default)]
pub struct CreateSessionEndpoint {
    pub permissions: Option<Vec<Permission>>,
}

impl Endpoint for CreateSessionEndpoint {
    fn method(&self) -> Method { Method::POST }

    fn endpoint(&self) -> String {
        "https://api.fishfish.gg/v1/users/@me/tokens".into()
    }

    fn body(&self) -> Result<Option<Body>, FishFishError> {
        let json = serde_json::ser::to_string(&self)
            .map_err(|_| FishFishError::SerializationError)?;

        Ok(Some(Body::from(json)))
    }
}

#[async_trait(?Send)]
impl AuthWebRequest<Token> for CreateSessionEndpoint {}
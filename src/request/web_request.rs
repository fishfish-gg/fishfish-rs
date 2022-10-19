use async_trait::async_trait;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use crate::client::client::{FishFishClient};
use crate::error::FishFishError;
use crate::request::Endpoint;

#[async_trait(?Send)]
pub trait RawWebRequest {
    async fn execute_raw(&self, client: &Client, session: Option<&String>) -> Result<Response, FishFishError>;
}

#[async_trait(?Send)]
pub trait AuthWebRequest<T: DeserializeOwned>: RawWebRequest {
    async fn execute(&self, client: &mut dyn FishFishClient) -> Result<T, FishFishError> {
        if client.has_token() {
            if client.is_token_expired()? {
                client.refresh_token().await?;
            }
        } else {
            return Err(FishFishError::NotAuthenticatedError);
        }

        self
            .execute_raw(&client.http(), client.get_session().map(|s| &s.token))
            .await?
            .json::<T>()
            .await
            .map_err(|e| FishFishError::ResponseParseFailed(e))
    }
}

#[async_trait(?Send)]
pub trait WebRequest<T: DeserializeOwned>: RawWebRequest {
    async fn execute(&self, client: &mut dyn FishFishClient) -> Result<T, FishFishError> {
        if client.has_token() && client.is_token_expired()? {
            client.refresh_token().await?;
        }

        self
            .execute_raw(&client.http(), client.get_session().map(|s| &s.token))
            .await?
            .json::<T>()
            .await
            .map_err(|e| FishFishError::ResponseParseFailed(e))
    }
}

#[async_trait(?Send)]
impl<T: Endpoint> RawWebRequest for T {

    async fn execute_raw(&self, client: &Client, session: Option<&String>) -> Result<Response, FishFishError> {
        let mut response = client
            .request(self.method(), self.endpoint())
            .query(&self.parameters());

        if let Some(body) = self.body()? {
            response = response.body(body);
        }

        if let Some(session) = session {
            response = response.header("Authorization", session);
        }

        let response = response
            .send()
            .await?;

        match response.status().as_u16() {
            401 => Err(FishFishError::InvalidTokenError)?,
            403 => Err(FishFishError::ForbiddenError)?,
            _ => {}
        }

        response
            .error_for_status()
            .map_err(|e| e.into())
    }
}
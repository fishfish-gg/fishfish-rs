use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use crate::data::{ApiStatus, Token};
use crate::client::AuthFishFishClient;
use crate::client::default_client::DefaultFishFishClient;
use crate::endpoints::create_session::CreateSessionEndpoint;
use crate::endpoints::GetStatusEndpoint;
use crate::error::FishFishError;
use crate::request::web_request::RawWebRequest;
use crate::request::WebRequest;


#[async_trait(?Send)]
pub trait FishFishRequestClient: FishFishClient + Sized {
    async fn get_status(&mut self) -> Result<ApiStatus, FishFishError> {
        self.invoke(GetStatusEndpoint {}).await
    }

    async fn invoke<Type: DeserializeOwned, R: WebRequest<Type>>(&mut self, request: R) -> Result<Type, FishFishError> {
        request.execute(self).await
    }
}

#[async_trait(?Send)]
pub trait FishFishClient {
    fn get_token(&self) -> Option<&String>;
    fn get_session(&self) -> Option<&Token>;
    fn http(&self) -> &Client;
    fn set_session(&mut self, _: Token) {}

    fn has_token(&self) -> bool {
        match &self.get_token() {
            None => false,
            Some(_) => true,
        }
    }

    fn is_token_expired(&self) -> Result<bool, FishFishError> {
        if let None = self.get_token() {
            return Ok(false);
        }

        let session = match self.get_session() {
            None => return Ok(true),
            Some(session) => session,
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| FishFishError::TimeWentBackwardsError)?;

        Ok(now.as_millis() as u64 > session.expires)
    }

    async fn refresh_token(&mut self) -> Result<(), FishFishError> {
        let create_session = CreateSessionEndpoint {
            permissions: None
        };

        let response = create_session
            .execute_raw(self.http(), self.get_token())
            .await?;

        let session = response
            .json::<Token>()
            .await
            .map_err(|e| FishFishError::TokenRefreshError(e))?;

        self.set_session(session);

        Ok(())
    }
}

pub struct FishFish;
impl FishFish {
    fn new_client() -> ClientBuilder {
        Client::builder()
            .https_only(true)
            .user_agent(format!("FishFish Rust Client/{}", env!("CARGO_PKG_VERSION")))
    }

    pub fn new() -> Result<DefaultFishFishClient, FishFishError> {
        let client = Self::new_client()
            .build()
            .map_err(|e| FishFishError::ClientInitializationError(e))?;

        Ok(DefaultFishFishClient {
            http: client
        })
    }

    pub fn new_with_token<T: ToString>(token: T) -> Result<AuthFishFishClient, FishFishError> {
        let client = Self::new_client()
            .build()
            .map_err(|e| FishFishError::ClientInitializationError(e))?;

        Ok(AuthFishFishClient {
            http: client,
            token: token.to_string(),
            session: None,
            permissions: None
        })
    }
}
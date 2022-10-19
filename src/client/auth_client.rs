use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::error::FishFishError;
use crate::data::{Domain, Token, DomainCategory};
use crate::client::{FishFishClient, FishFishRequestClient};
use crate::endpoints::{CreateDomainEndpoint, CreateSessionEndpoint};
use crate::request::web_request::AuthWebRequest;

pub struct AuthFishFishClient {
    pub(crate) http: Client,
    pub(crate) token: String,
    pub(crate) session: Option<Token>,
    pub(crate) permissions: Option<Vec<String>>,
}

impl FishFishRequestClient for AuthFishFishClient {}

#[allow(dead_code)]
impl AuthFishFishClient {
    pub async fn create_domain<S1: ToString, S2: ToString>(&mut self, domain: S1, category: DomainCategory, description: S2, target: Option<String>) -> Result<Domain, FishFishError> {
        self.invoke_auth(CreateDomainEndpoint {
            domain: domain.to_string(),
            description: description.to_string(),
            target,
            category,
        }).await
    }

    pub async fn create_session(&mut self, permissions: Option<Vec<String>>) -> Result<Token, FishFishError> {
        self.permissions = permissions.clone();
        self.invoke_auth(CreateSessionEndpoint { permissions }).await
    }

    pub(crate) async fn invoke_auth<Type: DeserializeOwned, Req: AuthWebRequest<Type>>(&mut self, request: Req) -> Result<Type, FishFishError> {
        request.execute(self).await
    }
}

impl FishFishClient for AuthFishFishClient {
    fn get_token(&self) -> Option<&String> {
        Some(&self.token)
    }

    fn get_session(&self) -> Option<&Token> {
        self.session.as_ref()
    }

    fn http(&self) -> &Client {
        &self.http
    }

    fn set_session(&mut self, session: Token) {
        self.session = Some(session);
    }
}
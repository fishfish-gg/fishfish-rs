use std::collections::HashSet;
use async_trait::async_trait;
use reqwest::{Method};
use serde::Serialize;
use crate::request::{Endpoint, WebRequest};

#[derive(Debug, Serialize, Clone)]
pub struct GetDomainsEndpoint {}

impl Endpoint for GetDomainsEndpoint {
    fn method(&self) -> Method { Method::GET }

    fn endpoint(&self) -> String {
        format!("https://api.fishfish.gg/v1/urls?full=false")
    }
}

#[async_trait(?Send)]
impl WebRequest<HashSet<String>> for GetDomainsEndpoint {}
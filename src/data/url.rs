use serde::{Deserialize, Serialize};
use crate::data::DomainCategory;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UrlEntry {
    url: String,
    category: Option<DomainCategory>,
    meta: Option<UrlMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UrlMetadata {
    path: Option<String>,
    urlscan: Option<String>,
    active: Option<String>,
    target: Option<String>,
}
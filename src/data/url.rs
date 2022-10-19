use serde::{Deserialize, Serialize};
use crate::DomainCategory;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UrlEntry {
    url: String,
    category: Option<DomainCategory>,
    meta: Option<UrlMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UrlMetadata {
    path: Option<String>,
    urlscan: Option<String>,
    active: Option<String>,
    target: Option<String>,
}
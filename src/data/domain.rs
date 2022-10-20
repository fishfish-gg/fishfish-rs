use serde::{Deserialize, Serialize};
use crate::data::DomainCategory;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Domain {
    name: String,
    description: String,
    category: DomainCategory,
    target: String,
    added: i64,
    checked: i64,
}
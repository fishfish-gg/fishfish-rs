use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Token {
    pub token: String,
    pub expires: u64,
}
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ApiStatus {
    domains: u64,
    urls: u64,
    worker: u64,
    uptime: u64,
    requests: u64,
}
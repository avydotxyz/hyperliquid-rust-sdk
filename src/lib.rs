#![deny(unreachable_pub)]
pub mod consts;
pub mod eip712;
pub mod errors;
pub mod exchange;
pub mod helpers;
pub mod info;
pub mod meta;
pub mod prelude;
pub mod req;
pub mod signature;
pub mod ws;

// Re-exports for convenience
pub use errors::Error;
pub use exchange::exchange_client::ExchangeClient;
pub use helpers::BaseUrl;
pub use info::info_client::InfoClient;
pub use req::HttpClient;

// Common types
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeResponseStatus {
    pub status: String,
    pub response: Option<serde_json::Value>,
}

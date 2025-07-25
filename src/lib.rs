#![deny(unreachable_pub)]
mod consts;
mod errors;
mod exchange;
mod helpers;
mod prelude;
mod proxy_digest;
pub mod signature;
pub mod ws;
pub use consts::{EPSILON, LOCAL_API_URL, MAINNET_API_URL, TESTNET_API_URL};
pub use errors::Error;
pub use exchange::*;
pub use helpers::{bps_diff, truncate_float, BaseUrl};
pub use ws::*;

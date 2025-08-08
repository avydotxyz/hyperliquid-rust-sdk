use alloy::primitives::B256;
use serde::{Deserialize, Serialize};

use crate::exchange::hash_generator::Actions;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageResponse {
    pub action: Actions,
    pub message: B256,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, utoipa::ToSchema)]
pub struct SpotTransferRequest {
    pub amount: String,
    pub destination: String,
    pub token: String,
}

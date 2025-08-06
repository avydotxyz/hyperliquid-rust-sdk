use alloy::primitives::B256;

use crate::{eip712::Eip712, prelude::*, signature::agent::l1};

#[cfg(not(feature = "testnet"))]
const SOURCE: &str = "a";

#[cfg(feature = "testnet")]
const SOURCE: &str = "b";

pub fn encode_l1_action(connection_id: B256) -> Result<B256> {
    let payload = l1::Agent {
        source: SOURCE.to_string(),
        connectionId: connection_id,
    };
    let encoded = payload.eip712_signing_hash();
    let action = B256::from(encoded);
    Ok(action)
}

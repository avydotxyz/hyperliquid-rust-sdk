use crate::{prelude::*, signature::agent::l1, Error};

use alloy::{
    primitives::B256,
    signers::{local::PrivateKeySigner, Signature, SignerSync},
};
#[cfg(not(feature = "testnet"))]
const SOURCE: &str = "a";

#[cfg(feature = "testnet")]
const SOURCE: &str = "b";

pub fn encode_l1_action(connection_id: B256) -> Result<B256> {
    let payload = &l1::Agent {
        source: SOURCE.to_string(),
        connectionId: connection_id,
    };
    let encoded = payload
        .eip712_signing_hash()
        .map_err(|e| Error::SignatureFailure(e.to_string()))?;

    let action = B256::from(encoded);
    Ok(action)
}

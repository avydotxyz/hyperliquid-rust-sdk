use alloy::{primitives::B256, signers::{local::PrivateKeySigner, Signature, SignerSync}};

use crate::{eip712::Eip712, errors::Error, prelude::*, signature::agent::l1};

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

pub(crate) fn sign_l1_action(
    wallet: &PrivateKeySigner,
    connection_id: B256,
    is_mainnet: bool,
) -> Result<Signature> {
    let source = if is_mainnet { "a" } else { "b" }.to_string();
    let payload = l1::Agent {
        source,
        connectionId: connection_id,
    };
    sign_typed_data(&payload, wallet)
}

pub(crate) fn sign_typed_data<T: Eip712>(
    payload: &T,
    wallet: &PrivateKeySigner,
) -> Result<Signature> {
    wallet
        .sign_hash_sync(&payload.eip712_signing_hash())
        .map_err(|e| Error::SignatureFailure(e.to_string()))
}

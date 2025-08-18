use std::collections::HashMap;

use alloy::{
    primitives::{Address, Signature},
    signers::local::PrivateKeySigner,
};
use log::debug;
use reqwest::Client;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{
    exchange::{actions::*, hash_generator::Actions},
    helpers::{next_nonce, BaseUrl},
    info::InfoClient,
    meta::Meta,
    prelude::*,
    req::HttpClient,
    signature::create_signature::sign_l1_action,
    Error, ExchangeResponseStatus,
};

#[derive(Debug)]
pub struct ExchangeClient {
    pub http_client: HttpClient,
    pub wallet: PrivateKeySigner,
    pub meta: Meta,
    pub vault_address: Option<Address>,
    pub coin_to_asset: HashMap<String, u32>,
}

fn serialize_sig<S>(sig: &Signature, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut state = s.serialize_struct("Signature", 3)?;
    state.serialize_field("r", &sig.r())?;
    state.serialize_field("s", &sig.s())?;
    state.serialize_field("v", &(27 + sig.v() as u64))?;
    state.end()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExchangePayload {
    action: serde_json::Value,
    #[serde(serialize_with = "serialize_sig")]
    signature: Signature,
    nonce: u64,
    vault_address: Option<Address>,
}

impl ExchangeClient {
    pub async fn new(
        info_client: Option<InfoClient>,
        wallet: PrivateKeySigner,
        base_url: Option<BaseUrl>,
        vault_address: Option<Address>,
        client: Option<Client>,
    ) -> Result<ExchangeClient> {
        let base_url = base_url.unwrap_or(BaseUrl::Mainnet);
        let client = client.unwrap_or_default();
        let info_client = match info_client {
            Some(client) => client,
            None => InfoClient::new(None, Some(base_url.clone())).await?,
        };
        let meta = info_client.meta().await?;
        let coin_to_asset = meta
            .universe
            .iter()
            .enumerate()
            .map(|(i, asset_info)| (asset_info.name.clone(), i as u32))
            .collect();

        Ok(ExchangeClient {
            wallet,
            meta,
            vault_address,
            http_client: HttpClient {
                client,
                base_url: base_url.get_url(),
            },
            coin_to_asset,
        })
    }

    async fn post(
        &self,
        action: serde_json::Value,
        signature: Signature,
        nonce: u64,
    ) -> Result<ExchangeResponseStatus> {
        let exchange_payload = ExchangePayload {
            action,
            signature,
            nonce,
            vault_address: self.vault_address,
        };
        let res = serde_json::to_string(&exchange_payload)
            .map_err(|e| Error::JsonParse(e.to_string()))?;
        debug!("Sending request {res:?}");

        let output = &self
            .http_client
            .post("/exchange", res)
            .await
            .map_err(|e| Error::JsonParse(e.to_string()))?;
        debug!("Response: {output}");
        serde_json::from_str(output).map_err(|e| Error::JsonParse(e.to_string()))
    }

    pub async fn perp_deploy_set_oracle(
        &self,
        dex: String,
        oracle_pxs: HashMap<String, String>,
        mark_pxs: Vec<HashMap<String, String>>,
    ) -> Result<ExchangeResponseStatus> {
        let wallet = &self.wallet;
        let timestamp = next_nonce();

        // Convert HashMap to sorted Vec<(String, String)> as expected by the API
        let mut oracle_pxs_wire: Vec<(String, String)> = oracle_pxs.into_iter().collect();
        oracle_pxs_wire.sort_by(|a, b| a.0.cmp(&b.0));

        let mark_pxs_wire: Vec<Vec<(String, String)>> = mark_pxs
            .into_iter()
            .map(|mark_px_map| {
                let mut mark_px_vec: Vec<(String, String)> = mark_px_map.into_iter().collect();
                mark_px_vec.sort_by(|a, b| a.0.cmp(&b.0));
                mark_px_vec
            })
            .collect();

        let set_oracle = SetOracle {
            dex,
            oracle_pxs: oracle_pxs_wire,
            mark_pxs: mark_pxs_wire,
        };

        let perp_deploy = PerpDeploy { set_oracle };

        let action = Actions::PerpDeploy(perp_deploy);
        let connection_id = action.hash(timestamp, self.vault_address)?;
        let action = serde_json::to_value(&action).map_err(|e| Error::JsonParse(e.to_string()))?;
        let is_mainnet = self.http_client.is_mainnet();
        let signature = sign_l1_action(wallet, connection_id, is_mainnet)?;

        self.post(action, signature, timestamp).await
    }
}

use crate::exchange::{cancel::CancelRequest, modify::ModifyRequest, order::OrderRequest};
pub(crate) use ethers::{
    abi::{encode, ParamType, Tokenizable},
    types::{
        transaction::{
            eip712,
            eip712::{encode_eip712_type, EIP712Domain, Eip712, Eip712Error},
        },
        H160, H256, U256,
    },
    utils::keccak256,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{cancel::CancelRequestCloid, BuilderInfo};

pub(crate) const HYPERLIQUID_EIP_PREFIX: &str = "HyperliquidTransaction:";

fn eip_712_domain(chain_id: U256) -> EIP712Domain {
    EIP712Domain {
        name: Some("HyperliquidSignTransaction".to_string()),
        version: Some("1".to_string()),
        chain_id: Some(chain_id),
        verifying_contract: Some(
            "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
        ),
        salt: None,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsdSend {
    pub signature_chain_id: U256,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl Eip712 for UsdSend {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}UsdSend"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("destination".to_string(), ParamType::String),
                ("amount".to_string(), ParamType::String),
                ("time".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            destination,
            amount,
            time,
        } = self;
        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(destination.clone().into_token()),
            encode_eip712_type(amount.clone().into_token()),
            encode_eip712_type(time.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgent {
    pub signature_chain_id: U256,
    pub hyperliquid_chain: String,
    pub agent_address: H160,
    pub agent_name: Option<String>,
    pub nonce: u64,
}

impl Eip712 for ApproveAgent {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}ApproveAgent"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("agentAddress".to_string(), ParamType::Address),
                ("agentName".to_string(), ParamType::String),
                ("nonce".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            agent_address,
            agent_name,
            nonce,
        } = self;
        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(agent_address.into_token()),
            encode_eip712_type(agent_name.clone().unwrap_or_default().into_token()),
            encode_eip712_type(nonce.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw3 {
    pub hyperliquid_chain: String,
    pub signature_chain_id: U256,
    pub amount: String,
    pub time: u64,
    pub destination: String,
}

impl Eip712 for Withdraw3 {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}Withdraw"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("destination".to_string(), ParamType::String),
                ("amount".to_string(), ParamType::String),
                ("time".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            amount,
            time,
            destination,
        } = self;
        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(destination.clone().into_token()),
            encode_eip712_type(amount.clone().into_token()),
            encode_eip712_type(time.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotSend {
    pub hyperliquid_chain: String,
    pub signature_chain_id: U256,
    pub destination: String,
    pub token: String,
    pub amount: String,
    pub time: u64,
}

impl Eip712 for SpotSend {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}SpotSend"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("destination".to_string(), ParamType::String),
                ("token".to_string(), ParamType::String),
                ("amount".to_string(), ParamType::String),
                ("time".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            destination,
            token,
            amount,
            time,
        } = self;
        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(destination.clone().into_token()),
            encode_eip712_type(token.clone().into_token()),
            encode_eip712_type(amount.clone().into_token()),
            encode_eip712_type(time.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
    pub class_transfer: ClassTransfer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
    pub amount: String,
    pub to_perp: bool,
    pub nonce: u64,
    pub hyperliquid_chain: String,
    pub signature_chain_id: U256,
}

impl Eip712 for ClassTransfer {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}UsdClassTransfer"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("amount".to_string(), ParamType::String),
                ("toPerp".to_string(), ParamType::Bool),
                ("nonce".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            amount,
            to_perp,
            nonce,
        } = self;
        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(amount.clone().into_token()),
            encode_eip712_type(to_perp.into_token()),
            encode_eip712_type(nonce.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

impl ClassTransfer {
    /// Returns the EIP-712 signing hash for this ClassTransfer
    pub fn eip712_signing_hash(&self) -> Result<H256, Eip712Error> {
        use ethers::utils::keccak256;

        let domain_hash = self.domain()?.separator();
        let struct_hash = self.struct_hash()?;

        let mut message = Vec::with_capacity(66);
        message.extend_from_slice(b"\x19\x01");
        message.extend_from_slice(&domain_hash);
        message.extend_from_slice(&struct_hash);

        Ok(H256(keccak256(message)))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendAsset {
    pub hyperliquid_chain: String,
    pub signature_chain_id: U256,
    pub destination: String,
    pub source_dex: String,
    pub destination_dex: String,
    pub token: String,
    pub amount: String,
    pub from_sub_account: String,
    pub nonce: u64,
}

impl SendAsset {
    pub fn eip712_signing_hash(&self) -> Result<H256, Eip712Error> {
        use ethers::utils::keccak256;

        let domain_hash = self.domain()?.separator();
        let struct_hash = self.struct_hash()?;

        let mut message = Vec::with_capacity(66);
        message.extend_from_slice(b"\x19\x01");
        message.extend_from_slice(&domain_hash);
        message.extend_from_slice(&struct_hash);

        Ok(H256(keccak256(message)))
    }
}

impl Eip712 for SendAsset {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}SendAsset"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("destination".to_string(), ParamType::String),
                ("sourceDex".to_string(), ParamType::String),
                ("destinationDex".to_string(), ParamType::String),
                ("token".to_string(), ParamType::String),
                ("amount".to_string(), ParamType::String),
                ("fromSubAccount".to_string(), ParamType::String),
                ("nonce".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            destination,
            source_dex,
            destination_dex,
            token,
            amount,
            from_sub_account,
            nonce,
        } = self;
        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(destination.clone().into_token()),
            encode_eip712_type(source_dex.clone().into_token()),
            encode_eip712_type(destination_dex.clone().into_token()),
            encode_eip712_type(token.clone().into_token()),
            encode_eip712_type(amount.clone().into_token()),
            encode_eip712_type(from_sub_account.clone().into_token()),
            encode_eip712_type(nonce.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: H160,
    pub is_deposit: bool,
    pub usd: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFee {
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
    pub signature_chain_id: U256,
    pub hyperliquid_chain: String,
}

impl Eip712 for ApproveBuilderFee {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(eip_712_domain(self.signature_chain_id))
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(eip712::make_type_hash(
            format!("{HYPERLIQUID_EIP_PREFIX}ApproveBuilderFee"),
            &[
                ("hyperliquidChain".to_string(), ParamType::String),
                ("maxFeeRate".to_string(), ParamType::String),
                ("builder".to_string(), ParamType::Address),
                ("nonce".to_string(), ParamType::Uint(64)),
            ],
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let Self {
            signature_chain_id: _,
            hyperliquid_chain,
            max_fee_rate,
            builder,
            nonce,
        } = self;

        // Parse builder as address
        let builder_address: H160 = builder
            .parse()
            .map_err(|_| Eip712Error::FromHexError(hex::FromHexError::InvalidStringLength))?;

        let items = vec![
            ethers::abi::Token::Uint(Self::type_hash()?.into()),
            encode_eip712_type(hyperliquid_chain.clone().into_token()),
            encode_eip712_type(max_fee_rate.clone().into_token()),
            encode_eip712_type(builder_address.into_token()),
            encode_eip712_type(nonce.into_token()),
        ];
        Ok(keccak256(encode(&items)))
    }
}

impl ApproveBuilderFee {
    /// Returns the EIP-712 signing hash for this ApproveBuilderFee
    pub fn eip712_signing_hash(&self) -> Result<H256, Eip712Error> {
        use ethers::utils::keccak256;

        let domain_hash = self.domain()?.separator();
        let struct_hash = self.struct_hash()?;

        let mut message = Vec::with_capacity(66);
        message.extend_from_slice(b"\x19\x01");
        message.extend_from_slice(&domain_hash);
        message.extend_from_slice(&struct_hash);

        Ok(H256(keccak256(message)))
    }
}

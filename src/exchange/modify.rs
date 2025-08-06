use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{order::OrderRequest, ClientOrderRequest};

#[derive(Debug, ToSchema)]
pub struct ClientModifyRequest {
    pub oid: u64,
    pub order: ClientOrderRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct ModifyRequest {
    pub oid: u64,
    pub order: OrderRequest,
}

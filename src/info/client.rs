use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::Result;
use crate::{helpers::BaseUrl, req::HttpClient, Error};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum InfoRequest {
    Meta {
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    #[serde(rename = "perpDeployAuctionStatus")]
    PerpDeployAuctionStatus,
}

#[derive(Debug, Clone)]
pub struct InfoClient {
    pub http_client: HttpClient,
}

impl InfoClient {
    pub fn new(base_url: BaseUrl) -> Self {
        Self {
            http_client: HttpClient {
                client: reqwest::Client::new(),
                base_url: base_url.get_url(),
            },
        }
    }

    async fn send_info_request<T: for<'a> Deserialize<'a>>(
        &self,
        info_request: InfoRequest,
    ) -> Result<T> {
        let data =
            serde_json::to_string(&info_request).map_err(|e| Error::JsonParse(e.to_string()))?;

        let return_data = self.http_client.post("/info", data).await?;
        serde_json::from_str(&return_data).map_err(|e| Error::JsonParse(e.to_string()))
    }

    /// Query perp deploy auction status which includes auction start and gas information
    pub async fn query_perp_deploy_auction_status(&self) -> Result<Value> {
        let input = InfoRequest::PerpDeployAuctionStatus;
        self.send_info_request(input).await
    }

    /// Get meta information for a specific dex
    pub async fn meta(&self, dex: Option<String>) -> Result<Value> {
        let input = InfoRequest::Meta { dex };
        self.send_info_request(input).await
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TransactionAssetAddRequest {
    pub asset_id: String,
    pub long_name: String,
    pub short_name: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionAssetAddResponse {
    pub id: i64,
    pub asset_id: String,
    pub long_name: String,
    pub short_name: String,
}

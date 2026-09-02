use tracing::{info, warn};

use crate::models::transaction_asset::{ TransactionAssetAddRequest, TransactionAssetAddResponse };
use crate::services::AppService;

impl AppService {
    pub async fn add_transaction_asset(
        &self,
        request: TransactionAssetAddRequest,
    ) -> Result<TransactionAssetAddResponse, String> {
        info!(asset_id = %request.asset_id, "Adding transaction asset");
        // ここで実際のビジネスロジックを実装
        // 例えば、データベースに保存するなど
        // 成功した場合はTransactionAssetAddResponseを返し、失敗した場合はエラーメッセージを返す
        Ok(TransactionAssetAddResponse {
            id: 1, // 仮のID
            asset_id: request.asset_id,
            long_name: request.long_name,
            short_name: request.short_name,
        })
    }
}
use super::TransactionInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcTransactionStatusCommonRequest {
    #[serde(flatten)]
    pub transaction_info: TransactionInfo,
}

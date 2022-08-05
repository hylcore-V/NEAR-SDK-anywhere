use {super::TransactionOrReceiptId, crate::hash::CryptoHash};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcLightClientExecutionProofRequest {
    #[serde(flatten)]
    pub id: TransactionOrReceiptId,
    pub light_client_head: CryptoHash,
}

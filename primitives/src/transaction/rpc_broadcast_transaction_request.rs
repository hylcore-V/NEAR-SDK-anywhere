use super::SignedTransaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcBroadcastTransactionRequest {
    #[serde(flatten)]
    pub signed_transaction: SignedTransaction,
}

use {super::StateChangesView, crate::hash::CryptoHash};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcStateChangesInBlockResponse {
    pub block_hash: CryptoHash,
    pub changes: StateChangesView,
}

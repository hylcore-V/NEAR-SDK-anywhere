use {super::StateChangesKindsView, crate::hash::CryptoHash};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcStateChangesInBlockByTypeResponse {
    pub block_hash: CryptoHash,
    pub changes: StateChangesKindsView,
}

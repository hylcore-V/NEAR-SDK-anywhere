use crate::types::BlockReference;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcStateChangesInBlockRequest {
    #[serde(flatten)]
    pub block_reference: BlockReference,
}

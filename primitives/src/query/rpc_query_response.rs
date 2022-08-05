use {super::QueryResponseKind, crate::types::BlockHeight, near_primitives_core::hash::CryptoHash};

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcQueryResponse {
    #[serde(flatten)]
    pub kind: QueryResponseKind,
    pub block_height: BlockHeight,
    pub block_hash: CryptoHash,
}

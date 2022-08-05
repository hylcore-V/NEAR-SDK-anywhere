use crate::{
    hash::CryptoHash,
    types::{BlockId, ShardId},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

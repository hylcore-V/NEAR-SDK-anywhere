use super::{BlockId, ShardId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockShardId {
    block_id: BlockId,
    shard_id: ShardId,
}

use {
    super::{EncodedShardChunkBody, ShardChunkHeader},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EncodedShardChunk {
    pub header: ShardChunkHeader,
    pub content: EncodedShardChunkBody,
}

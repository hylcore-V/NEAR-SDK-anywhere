use {
    super::{ChunkHash, ShardChunkHeader},
    crate::{receipt::Receipt, transaction::SignedTransaction},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Eq, PartialEq)]
pub struct ShardChunk {
    pub chunk_hash: ChunkHash,
    pub header: ShardChunkHeader,
    pub transactions: Vec<SignedTransaction>,
    pub receipts: Vec<Receipt>,
}

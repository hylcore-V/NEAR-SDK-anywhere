use {
    super::{EncodedShardChunk, ShardChunk},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub enum MaybeEncodedShardChunk {
    Encoded(EncodedShardChunk),
    Decoded(ShardChunk),
}

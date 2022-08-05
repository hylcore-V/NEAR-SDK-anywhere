use {
    super::MaybeEncodedShardChunk,
    crate::merkle::MerklePath,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub struct ChunkProofs {
    /// Encoded block header that contains invalid chunk.
    pub block_header: Vec<u8>,
    /// Merkle proof of inclusion of this chunk.
    pub merkle_proof: MerklePath,
    /// Invalid chunk in an encoded form or in a decoded form.
    pub chunk: MaybeEncodedShardChunk,
}

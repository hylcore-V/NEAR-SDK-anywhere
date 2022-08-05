use {
    super::BlockHeader,
    crate::{
        hash::CryptoHash,
        types::{BlockHeight, EpochId, MerkleHash},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct BlockHeaderInnerLiteView {
    /// Height of this block.
    pub height: BlockHeight,
    /// Epoch start hash of this block's epoch.
    /// Used for retrieving validator information
    pub epoch_id: EpochId,
    pub next_epoch_id: EpochId,
    /// Root hash of the state at the previous block.
    pub prev_state_root: MerkleHash,
    /// Root of the outcomes of transactions and receipts.
    pub outcome_root: MerkleHash,
    /// Timestamp at which the block was built (number of non-leap-nanoseconds since January 1, 1970 0:00:00 UTC).
    pub timestamp: u64,
    /// Hash of the next epoch block producers set
    pub next_bp_hash: CryptoHash,
    /// Merkle root of block hashes up to the current block.
    pub block_merkle_root: CryptoHash,
}

impl From<BlockHeader> for BlockHeaderInnerLiteView {
    fn from(header: BlockHeader) -> Self {
        BlockHeaderInnerLiteView {
            height: header.height,
            epoch_id: header.epoch_id,
            next_epoch_id: header.next_epoch_id,
            prev_state_root: header.prev_state_root,
            outcome_root: header.outcome_root,
            timestamp: header.timestamp,
            next_bp_hash: header.next_bp_hash,
            block_merkle_root: header.block_merkle_root,
        }
    }
}

use {
    crate::{
        challenge::ChallengesResult,
        hash::CryptoHash,
        network::ValidatorStake,
        types::{Balance, BlockHash, BlockHeight, EpochId, MerkleHash, ProtocolVersion},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::Signature,
    near_primitives_core::types::NumBlocks,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct BlockHeader {
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub signature: Signature,

    /// Number of chunks included into the block.
    pub chunks_included: u64,

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

    /// Root hash of the chunk receipts in the given block.
    pub chunk_receipts_root: MerkleHash,
    /// Root hash of the chunk headers in the given block.
    pub chunk_headers_root: MerkleHash,
    /// Root hash of the chunk transactions in the given block.
    pub chunk_tx_root: MerkleHash,
    /// Root hash of the challenges in the given block.
    pub challenges_root: MerkleHash,
    /// The output of the randomness beacon
    pub random_value: CryptoHash,
    /// Validator proposals.
    pub validator_proposals: Vec<ValidatorStake>,
    /// Mask for new chunks included in the block
    pub chunk_mask: Vec<bool>,
    /// Gas price. Same for all chunks
    pub gas_price: Balance,
    /// Total supply of tokens in the system
    pub total_supply: Balance,
    /// List of challenges result from previous block.
    pub challenges_result: ChallengesResult,

    /// Last block that has full BFT finality
    pub last_final_block: CryptoHash,
    /// Last block that has doomslug finality
    pub last_ds_final_block: CryptoHash,

    /// The ordinal of the Block on the Canonical Chain
    pub block_ordinal: NumBlocks,

    pub prev_height: BlockHeight,

    pub epoch_sync_data_hash: Option<CryptoHash>,

    /// All the approvals included in this block
    pub approvals: Vec<Option<Signature>>,

    /// Latest protocol version that this block producer has.
    pub latest_protocol_version: ProtocolVersion,
}

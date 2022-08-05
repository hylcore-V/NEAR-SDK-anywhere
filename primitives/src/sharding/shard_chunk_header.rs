use {
    super::ChunkHash,
    crate::{
        hash::CryptoHash,
        network::ValidatorStake,
        types::{Balance, BlockHeight, Gas, ShardId, StateRoot},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::Signature,
};

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct ShardChunkHeader {
    /// Previous block hash.
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: StateRoot,
    /// Root of the outcomes from execution transactions and results.
    pub outcome_root: CryptoHash,
    pub encoded_merkle_root: CryptoHash,
    pub encoded_length: u64,
    pub height_created: BlockHeight,
    /// Shard index.
    pub shard_id: ShardId,
    /// Gas used in this chunk.
    pub gas_used: Gas,
    /// Gas limit voted by validators.
    pub gas_limit: Gas,
    /// Total balance burnt in previous chunk
    pub balance_burnt: Balance,
    /// Outgoing receipts merkle root.
    pub outgoing_receipts_root: CryptoHash,
    /// Tx merkle root.
    pub tx_root: CryptoHash,
    /// Validator proposals.
    pub validator_proposals: Vec<ValidatorStake>,

    pub height_included: BlockHeight,

    /// Signature of the chunk producer.
    pub signature: Signature,

    #[borsh_skip]
    pub hash: ChunkHash,
}

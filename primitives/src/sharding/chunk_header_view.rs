use {
    super::{ChunkHash, ShardChunkHeader},
    crate::{network::ValidatorStakeView, serialize::u128_dec_format, types::StateRoot},
    near_anywhere_crypto::Signature,
    near_primitives_core::{
        hash::CryptoHash,
        types::{Balance, BlockHeight, Gas, ShardId},
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkHeaderView {
    pub chunk_hash: CryptoHash,
    pub prev_block_hash: CryptoHash,
    pub outcome_root: CryptoHash,
    pub prev_state_root: StateRoot,
    pub encoded_merkle_root: CryptoHash,
    pub encoded_length: u64,
    pub height_created: BlockHeight,
    pub height_included: BlockHeight,
    pub shard_id: ShardId,
    pub gas_used: Gas,
    pub gas_limit: Gas,
    /// TODO(2271): deprecated.
    #[serde(with = "u128_dec_format")]
    pub rent_paid: Balance,
    /// TODO(2271): deprecated.
    #[serde(with = "u128_dec_format")]
    pub validator_reward: Balance,
    #[serde(with = "u128_dec_format")]
    pub balance_burnt: Balance,
    pub outgoing_receipts_root: CryptoHash,
    pub tx_root: CryptoHash,
    pub validator_proposals: Vec<ValidatorStakeView>,
    pub signature: Signature,
}

impl From<ShardChunkHeader> for ChunkHeaderView {
    fn from(chunk: ShardChunkHeader) -> Self {
        Self {
            chunk_hash: chunk.hash.0,
            prev_block_hash: chunk.prev_block_hash,
            outcome_root: chunk.outcome_root,
            prev_state_root: chunk.prev_state_root,
            encoded_merkle_root: chunk.encoded_merkle_root,
            encoded_length: chunk.encoded_length,
            height_created: chunk.height_created,
            height_included: chunk.height_included,
            shard_id: chunk.shard_id,
            gas_used: chunk.gas_used,
            gas_limit: chunk.gas_limit,
            rent_paid: 0,
            validator_reward: 0,
            balance_burnt: chunk.balance_burnt,
            outgoing_receipts_root: chunk.outgoing_receipts_root,
            tx_root: chunk.tx_root,
            validator_proposals: chunk
                .validator_proposals
                .into_iter()
                .map(Into::into)
                .collect(),
            signature: chunk.signature,
        }
    }
}
impl From<ChunkHeaderView> for ShardChunkHeader {
    fn from(view: ChunkHeaderView) -> Self {
        ShardChunkHeader {
            prev_block_hash: view.prev_block_hash,
            prev_state_root: view.prev_state_root,
            outcome_root: view.outcome_root,
            encoded_merkle_root: view.encoded_merkle_root,
            encoded_length: view.encoded_length,
            height_created: view.height_created,
            shard_id: view.shard_id,
            gas_used: view.gas_used,
            gas_limit: view.gas_limit,
            balance_burnt: view.balance_burnt,
            outgoing_receipts_root: view.outgoing_receipts_root,
            tx_root: view.tx_root,
            validator_proposals: view
                .validator_proposals
                .into_iter()
                .map(Into::into)
                .collect(),
            height_included: view.height_included,
            signature: view.signature,
            hash: ChunkHash::default(),
        }
    }
}

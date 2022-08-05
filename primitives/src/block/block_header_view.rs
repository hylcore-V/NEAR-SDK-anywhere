use {
    super::BlockHeader,
    crate::{
        challenge::ChallengesResult,
        hash::CryptoHash,
        network::ValidatorStakeView,
        serialize::{u128_dec_format, u64_dec_format},
        types::{Balance, BlockHeight, EpochId, NumBlocks, ProtocolVersion},
    },
    near_anywhere_crypto::Signature,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeaderView {
    pub height: BlockHeight,
    pub prev_height: Option<BlockHeight>,
    pub epoch_id: EpochId,
    pub next_epoch_id: EpochId,
    pub hash: CryptoHash,
    pub prev_hash: CryptoHash,
    pub prev_state_root: CryptoHash,
    pub chunk_receipts_root: CryptoHash,
    pub chunk_headers_root: CryptoHash,
    pub chunk_tx_root: CryptoHash,
    pub outcome_root: CryptoHash,
    pub chunks_included: u64,
    pub challenges_root: CryptoHash,
    /// Legacy json number. Should not be used.
    pub timestamp: u64,
    #[serde(with = "u64_dec_format")]
    pub timestamp_nanosec: u64,
    pub random_value: CryptoHash,
    pub validator_proposals: Vec<ValidatorStakeView>,
    pub chunk_mask: Vec<bool>,
    #[serde(with = "u128_dec_format")]
    pub gas_price: Balance,
    pub block_ordinal: Option<NumBlocks>,
    /// TODO(2271): deprecated.
    #[serde(with = "u128_dec_format")]
    pub rent_paid: Balance,
    /// TODO(2271): deprecated.
    #[serde(with = "u128_dec_format")]
    pub validator_reward: Balance,
    #[serde(with = "u128_dec_format")]
    pub total_supply: Balance,
    pub challenges_result: ChallengesResult,
    pub last_final_block: CryptoHash,
    pub last_ds_final_block: CryptoHash,
    pub next_bp_hash: CryptoHash,
    pub block_merkle_root: CryptoHash,
    pub epoch_sync_data_hash: Option<CryptoHash>,
    pub approvals: Vec<Option<Signature>>,
    pub signature: Signature,
    pub latest_protocol_version: ProtocolVersion,
}

impl From<BlockHeader> for BlockHeaderView {
    fn from(header: BlockHeader) -> Self {
        Self {
            height: header.height,
            prev_height: Some(header.prev_height),
            epoch_id: header.epoch_id,
            next_epoch_id: header.next_epoch_id,
            hash: header.hash,
            prev_hash: header.prev_hash,
            prev_state_root: header.prev_state_root,
            chunk_receipts_root: header.chunk_receipts_root,
            chunk_headers_root: header.chunk_headers_root,
            chunk_tx_root: header.chunk_tx_root,
            chunks_included: header.chunks_included,
            challenges_root: header.challenges_root,
            outcome_root: header.outcome_root,
            timestamp: header.timestamp,
            timestamp_nanosec: header.timestamp,
            random_value: header.random_value,
            validator_proposals: header
                .validator_proposals
                .into_iter()
                .map(Into::into)
                .collect(),
            chunk_mask: header.chunk_mask.to_vec(),
            block_ordinal: if header.block_ordinal != 0 {
                Some(header.block_ordinal)
            } else {
                None
            },
            gas_price: header.gas_price,
            rent_paid: 0,
            validator_reward: 0,
            total_supply: header.total_supply,
            challenges_result: header.challenges_result.clone(),
            last_final_block: header.last_final_block,
            last_ds_final_block: header.last_ds_final_block,
            next_bp_hash: header.next_bp_hash,
            block_merkle_root: header.block_merkle_root,
            epoch_sync_data_hash: header.epoch_sync_data_hash,
            approvals: header.approvals.to_vec(),
            signature: header.signature.clone(),
            latest_protocol_version: header.latest_protocol_version,
        }
    }
}

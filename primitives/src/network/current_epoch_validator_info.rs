use {
    crate::{
        serialize::u128_dec_format,
        types::{AccountId, Balance, NumBlocks, ShardId},
    },
    near_anywhere_crypto::PublicKey,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CurrentEpochValidatorInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub is_slashed: bool,
    #[serde(with = "u128_dec_format")]
    pub stake: Balance,
    pub shards: Vec<ShardId>,
    pub num_produced_blocks: NumBlocks,
    pub num_expected_blocks: NumBlocks,
    #[serde(default)]
    pub num_produced_chunks: NumBlocks,
    #[serde(default)]
    pub num_expected_chunks: NumBlocks,
}

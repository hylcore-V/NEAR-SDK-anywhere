use {
    crate::{
        serialize::u128_dec_format,
        types::{AccountId, Balance, ShardId},
    },
    near_anywhere_crypto::PublicKey,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct NextEpochValidatorInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    #[serde(with = "u128_dec_format")]
    pub stake: Balance,
    pub shards: Vec<ShardId>,
}

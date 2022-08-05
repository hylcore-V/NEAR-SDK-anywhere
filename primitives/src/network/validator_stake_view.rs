use {
    super::ValidatorStake,
    crate::{
        serialize::u128_dec_format,
        types::{AccountId, Balance},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    #[serde(with = "u128_dec_format")]
    pub stake: Balance,
}

impl From<ValidatorStake> for ValidatorStakeView {
    fn from(stake: ValidatorStake) -> Self {
        Self {
            account_id: stake.account_id,
            public_key: stake.public_key,
            stake: stake.stake,
        }
    }
}

impl From<ValidatorStakeView> for ValidatorStake {
    fn from(view: ValidatorStakeView) -> Self {
        Self {
            account_id: view.account_id,
            public_key: view.public_key,
            stake: view.stake,
        }
    }
}

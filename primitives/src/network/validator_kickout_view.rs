use {
    crate::{
        serialize::u128_dec_format,
        types::{AccountId, Balance, NumBlocks},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ValidatorKickoutReason {
    Slashed,
    NotEnoughBlocks {
        produced: NumBlocks,
        expected: NumBlocks,
    },
    NotEnoughChunks {
        produced: NumBlocks,
        expected: NumBlocks,
    },
    Unstaked,
    NotEnoughStake {
        #[serde(with = "u128_dec_format", rename = "stake_u128")]
        stake: Balance,
        #[serde(with = "u128_dec_format", rename = "threshold_u128")]
        threshold: Balance,
    },
    DidNotGetASeat,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ValidatorKickoutView {
    pub account_id: AccountId,
    pub reason: ValidatorKickoutReason,
}

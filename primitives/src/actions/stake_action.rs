use {
    super::Action,
    crate::{serialize::u128_dec_format_compatible, types::Balance},
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};

/// An action which stakes singer_id tokens and setup's validator public key
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct StakeAction {
    /// Amount of tokens to stake.
    #[serde(with = "u128_dec_format_compatible")]
    pub stake: Balance,
    /// Validator key which will be used to sign transactions on behalf of singer_id
    pub public_key: PublicKey,
}

impl From<StakeAction> for Action {
    fn from(stake_action: StakeAction) -> Self {
        Self::Stake(stake_action)
    }
}

use {
    super::Action,
    crate::{serialize::u128_dec_format_compatible, types::Balance},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct TransferAction {
    #[serde(with = "u128_dec_format_compatible")]
    pub deposit: Balance,
}

impl From<TransferAction> for Action {
    fn from(transfer_action: TransferAction) -> Self {
        Self::Transfer(transfer_action)
    }
}

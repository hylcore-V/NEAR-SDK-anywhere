use {
    crate::types::AccountId,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SlashedValidator {
    pub account_id: AccountId,
    pub is_double_sign: bool,
}

impl SlashedValidator {
    pub fn new(account_id: AccountId, is_double_sign: bool) -> Self {
        SlashedValidator {
            account_id,
            is_double_sign,
        }
    }
}

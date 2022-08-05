use crate::types::AccountId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ValidatorInfo {
    pub account_id: AccountId,
    pub is_slashed: bool,
}

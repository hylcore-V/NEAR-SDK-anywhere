use {
    crate::{hash::CryptoHash, types::AccountId},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Hash, Clone, Debug, PartialEq, Eq,
)]
pub struct DataReceiver {
    pub data_id: CryptoHash,
    pub receiver_id: AccountId,
}

use {
    crate::{hash::CryptoHash, types::AccountId},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DataReceiverView {
    pub data_id: CryptoHash,
    pub receiver_id: AccountId,
}

use {
    crate::{
        hash::CryptoHash,
        types::{AccountId, Balance, BlockHeight, StorageUsage},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct AccountChanges {
    pub account_id: AccountId,
    pub amount: Balance,
    pub locked: Balance,
    pub code_hash: CryptoHash,
    pub storage_usage: StorageUsage,
    pub storage_paid_at: BlockHeight,
}

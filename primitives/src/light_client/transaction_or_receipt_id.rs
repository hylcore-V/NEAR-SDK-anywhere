use {
    crate::{hash::CryptoHash, types::AccountId},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, Clone, Debug, BorshDeserialize, BorshSerialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionOrReceiptId {
    Transaction {
        transaction_hash: CryptoHash,
        sender_id: AccountId,
    },
    Receipt {
        receipt_id: CryptoHash,
        receiver_id: AccountId,
    },
}

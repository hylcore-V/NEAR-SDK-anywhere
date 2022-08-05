use {
    super::SignedTransaction,
    crate::{hash::CryptoHash, types::AccountId},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionInfo {
    Transaction(SignedTransaction),
    TransactionId {
        hash: CryptoHash,
        account_id: AccountId,
    },
}

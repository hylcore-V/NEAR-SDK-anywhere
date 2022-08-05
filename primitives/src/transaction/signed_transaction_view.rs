use {
    super::SignedTransaction,
    crate::{
        actions::ActionView,
        hash::CryptoHash,
        types::{AccountId, Nonce},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::{PublicKey, Signature},
};

#[derive(Serialize, Deserialize, Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone)]
pub struct SignedTransactionView {
    pub signer_id: AccountId,
    pub public_key: PublicKey,
    pub nonce: Nonce,
    pub receiver_id: AccountId,
    pub actions: Vec<ActionView>,
    pub signature: Signature,
    pub hash: CryptoHash,
}

impl From<SignedTransaction> for SignedTransactionView {
    fn from(signed_tx: SignedTransaction) -> Self {
        let hash = signed_tx.get_hash();
        SignedTransactionView {
            signer_id: signed_tx.transaction.signer_id,
            public_key: signed_tx.transaction.public_key,
            nonce: signed_tx.transaction.nonce,
            receiver_id: signed_tx.transaction.receiver_id,
            actions: signed_tx
                .transaction
                .actions
                .into_iter()
                .map(|action| action.into())
                .collect(),
            signature: signed_tx.signature,
            hash,
        }
    }
}

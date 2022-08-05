use crate::{
    crypto::PublicKey,
    hash::CryptoHash,
    primitives::{actions::Action, transaction::Transaction},
};

pub fn create_transaction(
    signer_id: &str,
    public_key: PublicKey,
    receiver_id: &str,
    nonce: u64,
    actions: Vec<Action>,
    block_hash: CryptoHash,
) -> Transaction {
    let signer_id = signer_id.parse().unwrap();
    let receiver_id = receiver_id.parse().unwrap();

    Transaction {
        signer_id,
        public_key,
        nonce,
        receiver_id,
        actions,
        block_hash,
    }
}

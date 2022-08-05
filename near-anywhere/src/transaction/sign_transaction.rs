use {
    super::create_transaction,
    crate::{
        hash::CryptoHash,
        primitives::{
            actions::Action,
            transaction::{SignedTransaction, Transaction},
        },
        signer::Signer,
    },
    borsh::BorshSerialize,
};

pub async fn sign_transaction(
    transaction: Transaction,
    signer: Signer,
    account_id: &str,
    network_id: &str,
) -> SignedTransaction {
    let message = transaction
        .try_to_vec()
        .expect("fail to serialize the transaction");
    let signature = signer.sign_message(&message, account_id, network_id);

    let mut signed_transaction = SignedTransaction::new(signature, transaction);
    signed_transaction.init();

    signed_transaction
}

pub async fn sign_transaction_with_receiver(
    receiver_id: &str,
    nonce: u64,
    actions: Vec<Action>,
    block_hash: CryptoHash,
    signer: Signer,
    account_id: &str,
    network_id: &str,
) -> SignedTransaction {
    let public_key = signer.get_public_key(account_id, network_id).unwrap();
    let transaction = create_transaction(
        account_id,
        public_key,
        receiver_id,
        nonce,
        actions,
        block_hash,
    );
    let message = transaction
        .try_to_vec()
        .expect("fail to serialize the transaction");
    let signature = signer.sign_message(&message, account_id, network_id);
    let mut signed_transaction = SignedTransaction::new(signature, transaction);
    signed_transaction.init();

    signed_transaction
}

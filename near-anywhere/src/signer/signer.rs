use {
    super::InMemorySigner,
    crate::{
        crypto::{PublicKey, Signature},
        key_store::KeyStore,
    },
    std::io::Error,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Signer {
    InMemorySigner(InMemorySigner),
}

impl Signer {
    pub fn new_in_memory_signer(key_store: KeyStore) -> Self {
        Signer::InMemorySigner(InMemorySigner::new(key_store))
    }

    pub fn create_key(&self, account_id: &str, network_id: &str) -> PublicKey {
        match self {
            Signer::InMemorySigner(signer) => signer.create_key(account_id, network_id),
        }
    }

    pub fn get_public_key(&self, account_id: &str, network_id: &str) -> Result<PublicKey, Error> {
        match self {
            Signer::InMemorySigner(signer) => signer.get_public_key(account_id, network_id),
        }
    }

    pub fn sign_message(&self, message: &[u8], account_id: &str, network_id: &str) -> Signature {
        match self {
            Signer::InMemorySigner(signer) => signer.sign_message(message, account_id, network_id),
        }
    }
}

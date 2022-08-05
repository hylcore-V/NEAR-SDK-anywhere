use {
    crate::{
        crypto::{KeyType, PublicKey, Signature},
        key_pair::KeyPair,
        key_store::KeyStore,
    },
    sha2::Digest,
    std::io::{Error, ErrorKind},
};

#[derive(Debug, Clone)]
pub struct InMemorySigner {
    key_store: KeyStore,
}

impl InMemorySigner {
    pub fn new(key_store: KeyStore) -> Self {
        Self { key_store }
    }
}

impl InMemorySigner {
    pub fn create_key(&self, account_id: &str, network_id: &str) -> PublicKey {
        let key_pair = KeyPair::from_random(KeyType::ED25519);
        let public_key = key_pair.public_key();
        self.key_store.set_key(account_id, network_id, key_pair);

        public_key
    }

    pub fn from_key_pair(network_id: &str, account_id: &str, keypair: KeyPair) -> Self {
        let key_store = KeyStore::in_memory_key_store();

        key_store.set_key(network_id, account_id, keypair);

        Self { key_store }
    }

    pub fn get_public_key(&self, account_id: &str, network_id: &str) -> Result<PublicKey, Error> {
        let key_pair = self.key_store.get_key(account_id, network_id);

        match key_pair {
            Some(key_pair) => Ok(key_pair.public_key()),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "No key found for this account",
            )),
        }
    }

    pub fn sign_message(&self, message: &[u8], account_id: &str, network_id: &str) -> Signature {
        let hash = sha2::Sha256::digest(message);
        let hash = hash.as_slice();
        let key_pair = self
            .key_store
            .get_key(account_id, network_id)
            .expect("keypair not found");

        key_pair.sign(hash)
    }
}

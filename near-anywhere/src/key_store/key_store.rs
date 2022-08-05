#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
use super::BrowserLocalStorageKeyStore;
use {super::InMemoryKeyStore, crate::key_pair::KeyPair};

#[derive(Debug, Clone, PartialEq)]
pub enum KeyStore {
    /// A keystore that stores keys in a local storage.
    #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
    BrowserLocalStorage(BrowserLocalStorageKeyStore),
    InMemoryStorage(InMemoryKeyStore),
}

impl KeyStore {
    pub fn new() -> Self {
        Self::in_memory_key_store()
    }

    pub fn in_memory_key_store() -> Self {
        KeyStore::InMemoryStorage(InMemoryKeyStore::new())
    }

    #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
    pub fn browser_local_storage_key_store() -> Self {
        KeyStore::BrowserLocalStorage(BrowserLocalStorageKeyStore::new())
    }

    pub fn set_key(&self, network_id: &str, account_id: &str, keypair: KeyPair) {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::BrowserLocalStorage(key_store) => {
                key_store.set_key(network_id, account_id, keypair);
            }
            _ => {}
        }
    }

    pub fn get_key(&self, network_id: &str, account_id: &str) -> Option<KeyPair> {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::BrowserLocalStorage(key_store) => key_store.get_key(network_id, account_id),
            _ => None,
        }
    }

    pub fn remove_key(&self, network_id: &str, account_id: &str) {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::BrowserLocalStorage(key_store) => {
                key_store.remove_key(network_id, account_id);
            }
            _ => {}
        }
    }

    pub fn get_networks(&self) -> Vec<String> {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::BrowserLocalStorage(key_store) => key_store.get_networks(),
            _ => vec![],
        }
    }

    pub fn get_accounts(&self, network_id: &str) -> Vec<String> {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::BrowserLocalStorage(key_store) => key_store.get_accounts(network_id),
            _ => vec![],
        }
    }

    pub fn clear(&self) {
        match self {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            KeyStore::BrowserLocalStorage(key_store) => {
                key_store.clear();
            }
            _ => {}
        }
    }
}

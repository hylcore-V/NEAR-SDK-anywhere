#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
mod browser_local_storage;
mod in_memory;
mod key_store;

#[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
pub use browser_local_storage::BrowserLocalStorageKeyStore;
pub use {in_memory::InMemoryKeyStore, key_store::KeyStore};

const LOCAL_STORAGE_KEY_PREFIX: &str = "near-anywhere:keystore";

pub fn storage_key_for_secret_key(network_id: &str, account_id: &str) -> String {
    format!("{}:{}:{}", LOCAL_STORAGE_KEY_PREFIX, account_id, network_id)
}

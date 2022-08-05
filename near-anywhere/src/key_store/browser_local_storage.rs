use {
    super::{storage_key_for_secret_key, LOCAL_STORAGE_KEY_PREFIX},
    crate::key_pair::KeyPair,
    web_sys::Storage,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BrowserLocalStorageKeyStore {
    storage: Storage,
}

impl BrowserLocalStorageKeyStore {
    pub fn new() -> Self {
        let storage = crate::browser::local_storage();

        Self { storage }
    }
}

impl BrowserLocalStorageKeyStore {
    pub fn set_key(&self, network_id: &str, account_id: &str, keypair: KeyPair) {
        let key = storage_key_for_secret_key(network_id, account_id);

        self.storage
            .set_item(&key, keypair.to_string().as_str())
            .expect("failed to set key pair");
    }

    pub fn get_key(&self, network_id: &str, account_id: &str) -> Option<KeyPair> {
        let key = storage_key_for_secret_key(network_id, account_id);
        let value = self.storage.get_item(&key).unwrap();

        match value {
            Some(value) => Some(KeyPair::from_secret_key(&value).unwrap()),
            None => None,
        }
    }

    pub fn remove_key(&self, network_id: &str, account_id: &str) {
        let key = storage_key_for_secret_key(network_id, account_id);
        self.storage.remove_item(&key).unwrap();
    }

    pub fn get_networks(&self) -> Vec<String> {
        let mut result = vec![];
        let keys = self.storage_keys();

        for key in keys {
            let parts = key.split(":").collect::<Vec<&str>>();
            if parts.len() == 4 {
                result.push(parts[3].to_string());
            }
        }

        result
    }

    pub fn get_accounts(&self, network_id: &str) -> Vec<String> {
        let mut result = vec![];
        let keys = self.storage_keys();

        for key in keys {
            let parts = key.split(":").collect::<Vec<&str>>();
            if parts.len() == 4 && parts[3] == network_id {
                result.push(parts[1].to_string());
            }
        }

        result
    }

    pub fn clear(&self) {
        let keys = self.storage_keys();

        for key in keys {
            self.storage.remove_item(&key).unwrap();
        }
    }

    fn storage_keys(&self) -> Vec<String> {
        let len = self.storage.length().unwrap();
        let mut keys = vec![];

        for i in 0..len {
            let key = self.storage.key(i).unwrap();
            match key {
                Some(key) => {
                    if key.starts_with(LOCAL_STORAGE_KEY_PREFIX) {
                        keys.push(key.to_string());
                    }
                }
                None => {}
            }
        }

        keys
    }
}

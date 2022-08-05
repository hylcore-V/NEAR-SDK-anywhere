use {super::storage_key_for_secret_key, crate::key_pair::KeyPair, hashbrown::HashMap};

#[derive(Debug, Clone)]
pub struct InMemoryKeyStore {
    storage: HashMap<String, String>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl InMemoryKeyStore {
    pub fn set_key(&mut self, network_id: &str, account_id: &str, keypair: &KeyPair) {
        let key = storage_key_for_secret_key(network_id, account_id);

        self.storage
            .insert(key, keypair.to_string())
            .expect("failed to set key pair");
    }

    pub fn get_key(&self, network_id: &str, account_id: &str) -> Option<KeyPair> {
        let key = storage_key_for_secret_key(network_id, account_id);
        let value = self.storage.get(&key);

        match value {
            Some(value) => Some(KeyPair::from_secret_key(&value).unwrap()),
            None => None,
        }
    }

    pub fn remove_key(&mut self, network_id: &str, account_id: &str) {
        let key = storage_key_for_secret_key(network_id, account_id);
        self.storage.remove(&key).unwrap();
    }

    pub fn get_networks(&self) -> Vec<String> {
        let mut result = vec![];
        let keys = self.storage.keys();

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
        let keys = self.storage.keys();

        for key in keys {
            let parts = key.split(":").collect::<Vec<&str>>();
            if parts.len() == 4 && parts[3] == network_id {
                result.push(parts[1].to_string());
            }
        }

        result
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }
}

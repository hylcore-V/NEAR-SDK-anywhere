use {
    crate::{client::Transport, key_store::KeyStore, signer::Signer},
    hashbrown::HashMap,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NearConfig {
    pub explorer_url: Option<String>,
    pub headers: HashMap<String, String>,
    pub helper_url: Option<String>,
    pub initial_balance: Option<String>,
    pub jsvm_account_id: Option<String>,
    pub network_id: Option<String>,
    pub node_url: String,
    pub key_store: KeyStore,
    pub master_account: Option<String>,
    pub signer: Option<Signer>,
    pub transport: Option<Transport>,
    pub wallet_url: Option<String>,
}

impl NearConfig {
    pub fn new(node_url: &str, key_store: KeyStore) -> Self {
        Self {
            explorer_url: None,
            headers: HashMap::new(),
            helper_url: None,
            initial_balance: None,
            jsvm_account_id: None,
            network_id: None,
            node_url: node_url.to_string(),
            key_store,
            master_account: None,
            signer: None,
            transport: None,
            wallet_url: None,
        }
    }
}

use {
    super::{AuthData, Near},
    crate::{
        account::Account,
        crypto::KeyType,
        key_pair::KeyPair,
        key_store::KeyStore,
        primitives::{transaction::Transaction, types::AccountId},
        serialize::to_base64,
        signer::Signer,
    },
    borsh::BorshSerialize,
    log::info,
    std::collections::HashMap,
    url::Url,
};

const LOGIN_WALLET_URL_SUFFIX: &str = "/login/";
const MULTISIG_HAS_METHOD: &str = "add_request_and_confirm";
const LOCAL_STORAGE_KEY_SUFFIX: &str = "_wallet_auth_key";
const PENDING_ACCESS_KEY_PREFIX: &str = "pending_key";

#[derive(Debug, Clone, PartialEq)]
pub struct WalletConnection {
    auth_data_key: String,
    near: Near,
    wallet_base_url: String,
    key_store: KeyStore,
    auth_data: Option<AuthData>,
    network_id: String,
    connected_account: Option<Account>,
}

impl WalletConnection {
    pub fn new(near: Near) -> Self {
        Self::new_with_prefix(near, "")
    }

    pub fn new_with_prefix(near: Near, prefix: &str) -> Self {
        let near = near.clone();
        let config = near.config.clone();
        let auth_data_key = format!("{}{}", prefix, LOCAL_STORAGE_KEY_SUFFIX);
        let wallet_base_url = config.wallet_url.clone().unwrap_or("".to_string());
        let network_id = config.network_id.unwrap_or("".to_string());
        let auth_data = {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            {
                let storage = crate::browser::local_storage();
                let data = storage.get_item(&auth_data_key).unwrap();
                match data {
                    Some(data) => {
                        let data: AuthData = serde_json::from_str(&data).unwrap();

                        Some(data)
                    }
                    _ => None,
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            None
        };
        let key_store = match near.connection.signer.clone() {
            Signer::InMemorySigner(signer) => signer.key_store,
            _ => KeyStore::new(),
        };

        Self {
            near,
            wallet_base_url,
            key_store,
            auth_data_key,
            auth_data,
            network_id,
            connected_account: None,
        }
    }

    pub fn get_account_id(&self) -> Option<AccountId> {
        match self.auth_data.clone() {
            Some(auth_data) => Some(auth_data.account_id),
            None => None,
        }
    }

    pub fn account(&mut self) -> Option<Account> {
        match (self.connected_account.clone(), self.auth_data.clone()) {
            (Some(account), _) => Some(account),
            (_, Some(auth_data)) => {
                let connection = self.near.connection.clone();
                let account_id = auth_data.clone().account_id;
                let account = Account::new(connection, account_id);

                self.connected_account = Some(account);
                self.connected_account.clone()
            }
            _ => None,
        }
    }

    pub fn is_signed_in(&self) -> bool {
        match &self.auth_data {
            Some(auth_data) => !auth_data.account_id.is_empty(),
            None => false,
        }
    }

    pub fn sign_out(&mut self) {
        self.auth_data = None;
        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            crate::browser::local_storage().remove_item(&self.auth_data_key);
        }
    }

    pub async fn request_sign_in(
        &mut self,
        contract_id: Option<String>,
        method_names: Option<Vec<String>>,
        success_url: Option<String>,
        failure_url: Option<String>,
    ) {
        let mut url = Url::parse(&format!(
            "{}{}",
            self.wallet_base_url, LOGIN_WALLET_URL_SUFFIX
        ))
        .unwrap();
        if let Some(contract_id) = contract_id {
            let contract_account = self.near.account(&contract_id);
            contract_account.state().await.unwrap();

            url.query_pairs_mut()
                .append_pair("contract_id", &contract_id);
            let key_pair = KeyPair::from_random(KeyType::ED25519);
            let public_key = key_pair.public_key().to_string();
            url.query_pairs_mut().append_pair("public_key", &public_key);

            let network_id = self.network_id.clone();
            let temp_key = format!("{}{}", PENDING_ACCESS_KEY_PREFIX, public_key);

            self.key_store.set_key(&network_id, &temp_key, key_pair);
        }
        if let Some(method_names) = method_names {
            for method_name in method_names {
                url.query_pairs_mut()
                    .append_pair("methodNames", &method_name);
            }
        }
        if let Some(success_url) = success_url {
            url.query_pairs_mut()
                .append_pair("success_url", &success_url);
        } else {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            {
                let current_url = crate::browser::current_url();
                url.query_pairs_mut()
                    .append_pair("success_url", &current_url);
            }
        }
        if let Some(failure_url) = failure_url {
            url.query_pairs_mut()
                .append_pair("failure_url", &failure_url);
        } else {
            #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
            {
                let current_url = crate::browser::current_url();
                url.query_pairs_mut()
                    .append_pair("failure_url", &current_url);
            }
        }

        let url = url.to_string();

        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            crate::browser::location().assign(&url);
        }
    }

    pub async fn request_sign_transactions(
        &self,
        transactions: Vec<Transaction>,
        callback_url: Option<String>,
        meta: Option<String>,
    ) {
        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            let current_url = crate::browser::current_url();
            let callback_url = callback_url.unwrap_or(current_url);
            let sign_url = format!("{}/sign", self.wallet_base_url);
            let mut new_url = url::Url::parse(&sign_url).unwrap();
            let transactions = transactions
                .iter()
                .map(|t| to_base64(t.try_to_vec().unwrap()))
                .collect::<Vec<String>>();

            let transaction_data = transactions.join(",");

            new_url
                .query_pairs_mut()
                .append_pair("transactions", &transaction_data);

            new_url
                .query_pairs_mut()
                .append_pair("callbackUrl", &callback_url);
            if let Some(meta) = meta {
                new_url.query_pairs_mut().append_pair("meta", &meta);
            }
            let new_url = new_url.to_string();

            crate::browser::location().assign(&new_url);
        }
    }

    fn _move_key_from_temp_to_permanent(&self, account_id: &str, public_key: &str) {
        let temp_key = format!("{}{}", PENDING_ACCESS_KEY_PREFIX, public_key);
        let network_id = self.network_id.clone();
        let key_pair = self.key_store.get_key(&network_id, &temp_key);
        if let Some(key_pair) = key_pair {
            self.key_store.set_key(&network_id, account_id, key_pair);
            self.key_store.remove_key(&network_id, &temp_key);
        }
    }

    pub fn complete_sign_in_with_access_key(&mut self) {
        #[cfg(all(target_arch = "wasm32", feature = "local_storage"))]
        {
            let storage = crate::browser::local_storage();
            let url = crate::browser::current_url();
            let mut parsed_url = Url::parse(&url).unwrap();
            let count = parsed_url.query_pairs().count();
            let query_pairs = parsed_url.query_pairs().into_owned();

            if count > 0 {
                let params = query_pairs.collect::<HashMap<String, String>>();
                let auth_data = AuthData::parse(params.clone());
                let result = serde_json::to_string(&auth_data).unwrap();
                let account_id = auth_data.account_id.clone();
                storage
                    .set_item(&self.auth_data_key, &result)
                    .expect("fail to save data");
                if let Some(public_key) = params.clone().get("public_key") {
                    self._move_key_from_temp_to_permanent(&account_id, public_key);
                }
                self.auth_data = Some(auth_data);
                let mut new_params = params.clone();
                new_params.remove("public_key");
                new_params.remove("all_keys");
                new_params.remove("account_id");
                new_params.remove("meta");
                new_params.remove("transactionHashes");
                parsed_url.query_pairs_mut().clear();

                if !new_params.is_empty() {
                    parsed_url.query_pairs_mut().extend_pairs(new_params.iter());
                } else {
                    parsed_url.set_query(None);
                }

                crate::browser::replace_url(parsed_url.as_str());
            }
        }
    }
}

use {
    crate::{crypto::PublicKey, near::Connection},
    std::collections::HashMap,
};

#[derive(Clone)]
pub struct UrlAccountCreator {
    connection: Connection,
    help_url: String,
}

impl UrlAccountCreator {
    pub fn new(connection: Connection, help_url: String) -> Self {
        Self {
            connection,
            help_url,
        }
    }

    pub async fn create_account(&self, account_id: &str, public_key: PublicKey) {
        let public_key = public_key.to_string();
        let mut data = HashMap::new();
        data.insert("newAccountId", account_id);
        data.insert("newAccountPublicKey", &public_key);

        let client = reqwest::Client::new();
        let res = client.post(&self.help_url).json(&data).send().await;
    }
}

use {crate::primitives::types::AccountId, std::collections::HashMap};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub account_id: AccountId,
    pub all_keys: Vec<String>,
}

impl AuthData {
    pub fn parse(map: HashMap<String, String>) -> Self {
        let account_id = map.get("account_id").unwrap().to_string().parse().unwrap();
        let all_keys = map
            .get("all_keys")
            .unwrap()
            .to_string()
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Self {
            account_id,
            all_keys,
        }
    }
}

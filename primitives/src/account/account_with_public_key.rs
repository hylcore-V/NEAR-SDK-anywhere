use {crate::types::AccountId, near_anywhere_crypto::PublicKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountWithPublicKey {
    pub account_id: AccountId,
    pub public_key: PublicKey,
}

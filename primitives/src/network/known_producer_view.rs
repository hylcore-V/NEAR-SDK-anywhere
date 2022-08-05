use {crate::types::AccountId, near_anywhere_crypto::PublicKey};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct KnownProducerView {
    pub account_id: AccountId,
    pub peer_id: PublicKey,
    pub next_hops: Option<Vec<PublicKey>>,
}

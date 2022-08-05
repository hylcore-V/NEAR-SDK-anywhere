use {
    super::RawStateChange,
    crate::trie_key::TrieKey,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct RawStateChangesWithTrieKey {
    pub trie_key: TrieKey,
    pub changes: Vec<RawStateChange>,
}

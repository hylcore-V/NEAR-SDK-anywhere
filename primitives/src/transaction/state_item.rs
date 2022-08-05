use {
    crate::types::TrieProofPath,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StateItem {
    pub key: String,
    pub value: String,
    pub proof: TrieProofPath,
}

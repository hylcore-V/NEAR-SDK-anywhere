use {
    super::StateItem,
    crate::types::TrieProofPath,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ViewStateResult {
    pub values: Vec<StateItem>,
    pub proof: TrieProofPath,
}

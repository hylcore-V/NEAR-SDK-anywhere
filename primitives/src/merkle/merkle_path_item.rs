use {
    super::Direction,
    crate::types::MerkleHash,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MerklePathItem {
    pub hash: MerkleHash,
    pub direction: Direction,
}

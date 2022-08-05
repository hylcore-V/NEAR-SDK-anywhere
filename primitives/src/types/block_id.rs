use {
    crate::{hash::CryptoHash, types::BlockHeight},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockId {
    Height(BlockHeight),
    Hash(CryptoHash),
}

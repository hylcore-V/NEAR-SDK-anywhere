use {
    super::QueryResponseKind,
    crate::{hash::CryptoHash, types::BlockHeight},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct QueryResponse {
    pub kind: QueryResponseKind,
    pub block_height: BlockHeight,
    pub block_hash: CryptoHash,
}

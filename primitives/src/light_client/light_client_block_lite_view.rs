use {
    crate::{
        block::{BlockHeader, BlockHeaderInnerLiteView},
        hash::{hash, CryptoHash},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientBlockLiteView {
    pub prev_block_hash: CryptoHash,
    pub inner_rest_hash: CryptoHash,
    pub inner_lite: BlockHeaderInnerLiteView,
}

impl From<BlockHeader> for LightClientBlockLiteView {
    fn from(header: BlockHeader) -> Self {
        Self {
            prev_block_hash: header.prev_hash,
            inner_rest_hash: hash(&header.try_to_vec().unwrap()),
            inner_lite: header.into(),
        }
    }
}

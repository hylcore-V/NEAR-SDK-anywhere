use {
    super::{Block, BlockHeaderView},
    crate::sharding::ChunkHeaderView,
    near_primitives_core::types::AccountId,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockView {
    pub author: AccountId,
    pub header: BlockHeaderView,
    pub chunks: Vec<ChunkHeaderView>,
}

impl BlockView {
    pub fn from_author_block(author: AccountId, block: Block) -> Self {
        BlockView {
            author,
            header: block.header.clone().into(),
            chunks: block.chunks.iter().cloned().map(Into::into).collect(),
        }
    }
}

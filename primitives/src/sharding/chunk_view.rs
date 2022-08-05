use {
    super::{ChunkHeaderView, ShardChunk},
    crate::{receipt::ReceiptView, transaction::SignedTransactionView},
    near_primitives_core::types::AccountId,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChunkView {
    pub author: AccountId,
    pub header: ChunkHeaderView,
    pub transactions: Vec<SignedTransactionView>,
    pub receipts: Vec<ReceiptView>,
}

impl ChunkView {
    pub fn from_author_chunk(author: AccountId, chunk: ShardChunk) -> Self {
        Self {
            author,
            header: chunk.header.into(),
            transactions: chunk.transactions.into_iter().map(Into::into).collect(),
            receipts: chunk.receipts.into_iter().map(Into::into).collect(),
        }
    }
}

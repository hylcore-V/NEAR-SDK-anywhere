use {
    crate::{
        hash::{hash, CryptoHash},
        types::{AccountId, BlockHeader,},
        block::BlockHeaderInnerLiteView,
        merkle::MerklePath,
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, Clone, Debug, BorshDeserialize, BorshSerialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionOrReceiptId {
    Transaction {
        transaction_hash: CryptoHash,
        sender_id: AccountId,
    },
    Receipt {
        receipt_id: CryptoHash,
        receiver_id: AccountId,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientProof {
    pub outcome_root_proof: MerklePath,
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: MerklePath,
}

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientProofRequest {
    #[serde(flatten)]
    id: TransactionOrReceiptId,
    light_client_head: CryptoHash,
}

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientBlockLiteView {
    pub prev_block_hash: CryptoHash,
    pub inner_rest_hash: CryptoHash,
    pub inner_lite: BlockHeaderInnerLiteView,
}

impl From<BlockHeader> for LightClientBlockLiteView {
    fn from(header: BlockHeader) -> Self {
        Self {
            prev_block_hash: *header.prev_hash(),
            inner_rest_hash: hash(&header.inner_rest_bytes()),
            inner_lite: header.into(),
        }
    }
}

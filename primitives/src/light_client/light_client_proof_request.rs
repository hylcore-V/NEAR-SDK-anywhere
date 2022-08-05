use {
    super::TransactionOrReceiptId,
    crate::hash::CryptoHash,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientProofRequest {
    #[serde(flatten)]
    id: TransactionOrReceiptId,
    light_client_head: CryptoHash,
}

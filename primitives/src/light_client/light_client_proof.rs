use {
    super::LightClientBlockLiteView,
    crate::merkle::MerklePath,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct LightClientProof {
    pub outcome_root_proof: MerklePath,
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: MerklePath,
}

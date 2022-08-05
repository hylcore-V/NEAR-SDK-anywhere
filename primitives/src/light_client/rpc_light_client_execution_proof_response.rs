use {
    super::LightClientBlockLiteView,
    crate::{merkle::MerklePath, transaction::ExecutionOutcomeWithIdView},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcLightClientExecutionProofResponse {
    pub outcome_proof: ExecutionOutcomeWithIdView,
    pub outcome_root_proof: MerklePath,
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: MerklePath,
}

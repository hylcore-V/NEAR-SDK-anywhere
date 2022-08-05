use {super::StateChangesRequestView, crate::types::BlockReference};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcStateChangesInBlockByTypeRequest {
    #[serde(flatten)]
    pub block_reference: BlockReference,
    #[serde(flatten)]
    pub state_changes_request: StateChangesRequestView,
}

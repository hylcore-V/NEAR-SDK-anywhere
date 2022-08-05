use {super::QueryRequest, crate::types::BlockReference};

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcQueryRequest {
    #[serde(flatten)]
    pub block_reference: BlockReference,
    #[serde(flatten)]
    pub request: QueryRequest,
}

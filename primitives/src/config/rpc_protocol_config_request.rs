use crate::types::BlockReference;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcProtocolConfigRequest {
    #[serde(flatten)]
    pub block_reference: BlockReference,
}

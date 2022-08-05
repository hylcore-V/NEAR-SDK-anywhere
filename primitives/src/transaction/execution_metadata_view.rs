use {
    super::ExecutionMetadata,
    crate::types::CostGasUsed,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
pub struct ExecutionMetadataView {
    pub version: u32,
    pub gas_profile: Option<Vec<CostGasUsed>>,
}

impl From<ExecutionMetadata> for ExecutionMetadataView {
    fn from(metadata: ExecutionMetadata) -> Self {
        ExecutionMetadataView {
            version: metadata.version,
            gas_profile: metadata.gas_profile,
        }
    }
}

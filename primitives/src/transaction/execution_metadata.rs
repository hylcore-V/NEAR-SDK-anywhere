use {
    crate::types::CostGasUsed,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default, PartialEq, Clone, Eq, Debug,
)]
pub struct ExecutionMetadata {
    pub version: u32,
    pub gas_profile: Option<Vec<CostGasUsed>>,
}

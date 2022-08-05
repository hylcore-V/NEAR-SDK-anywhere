use {
    crate::{serialize::u64_dec_format, types::Gas},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
pub struct CostGasUsed {
    pub cost_category: String,
    pub cost: String,
    #[serde(with = "u64_dec_format")]
    pub gas_used: Gas,
}

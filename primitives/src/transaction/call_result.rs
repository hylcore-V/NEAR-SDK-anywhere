use borsh::{BorshDeserialize, BorshSerialize};

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default,
)]
pub struct CallResult {
    pub result: Vec<u8>,
    pub logs: Vec<String>,
}

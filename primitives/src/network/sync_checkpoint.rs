use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncCheckpoint {
    Genesis,
    EarliestAvailable,
}

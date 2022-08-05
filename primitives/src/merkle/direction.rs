use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
}

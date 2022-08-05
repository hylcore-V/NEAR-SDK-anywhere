use {
    super::StateItem,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct PartialState(pub Vec<StateItem>);

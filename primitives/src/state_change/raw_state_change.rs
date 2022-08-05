use {
    super::StateChangeCause,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct RawStateChange {
    pub cause: StateChangeCause,
    pub data: Option<Vec<u8>>,
}

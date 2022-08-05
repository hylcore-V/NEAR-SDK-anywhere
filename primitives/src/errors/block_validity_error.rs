use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
pub enum BlockValidityError {
    InvalidStateRoot,
    InvalidReceiptRoot,
    InvalidChunkHeaderRoot,
    InvalidTransactionRoot,
    InvalidChunkMask,
    InvalidChallengeRoot,
}

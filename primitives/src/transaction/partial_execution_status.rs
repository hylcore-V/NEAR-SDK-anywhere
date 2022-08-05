use {
    super::ExecutionStatus,
    crate::hash::CryptoHash,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Clone)]
pub enum PartialExecutionStatus {
    Unknown,
    Failure,
    SuccessValue(Vec<u8>),
    SuccessReceiptId(CryptoHash),
}

impl From<ExecutionStatus> for PartialExecutionStatus {
    fn from(status: ExecutionStatus) -> PartialExecutionStatus {
        match status {
            ExecutionStatus::Unknown => PartialExecutionStatus::Unknown,
            ExecutionStatus::Failure(_) => PartialExecutionStatus::Failure,
            ExecutionStatus::SuccessValue(value) => PartialExecutionStatus::SuccessValue(value),
            ExecutionStatus::SuccessReceiptId(id) => PartialExecutionStatus::SuccessReceiptId(id),
        }
    }
}

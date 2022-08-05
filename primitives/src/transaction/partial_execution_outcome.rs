use {
    super::{ExecutionOutcome, PartialExecutionStatus},
    crate::{
        hash::CryptoHash,
        types::{AccountId, Balance, Gas},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Clone)]
pub struct PartialExecutionOutcome {
    pub receipt_ids: Vec<CryptoHash>,
    pub gas_burnt: Gas,
    pub tokens_burnt: Balance,
    pub executor_id: AccountId,
    pub status: PartialExecutionStatus,
}

impl From<&ExecutionOutcome> for PartialExecutionOutcome {
    fn from(outcome: &ExecutionOutcome) -> Self {
        Self {
            receipt_ids: outcome.receipt_ids.clone(),
            gas_burnt: outcome.gas_burnt,
            tokens_burnt: outcome.tokens_burnt,
            executor_id: outcome.executor_id.clone(),
            status: outcome.status.clone().into(),
        }
    }
}

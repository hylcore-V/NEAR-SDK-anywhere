use {
    super::{ExecutionOutcomeWithIdView, FinalExecutionStatus, SignedTransaction},
    crate::logging,
    borsh::{BorshDeserialize, BorshSerialize},
    core::fmt,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FinalExecutionOutcome {
    /// Execution status. Contains the result in case of successful execution.
    pub status: FinalExecutionStatus,
    /// Signed Transaction
    pub transaction: SignedTransaction,
    /// The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
    /// The execution outcome of receipts.
    pub receipts_outcome: Vec<ExecutionOutcomeWithIdView>,
}

impl fmt::Debug for FinalExecutionOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FinalExecutionOutcome")
            .field("status", &self.status)
            .field("transaction", &self.transaction)
            .field("transaction_outcome", &self.transaction_outcome)
            .field(
                "receipts_outcome",
                &format_args!("{}", logging::pretty_vec(&self.receipts_outcome)),
            )
            .finish()
    }
}

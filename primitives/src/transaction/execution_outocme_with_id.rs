use {
    super::ExecutionOutcome,
    crate::hash::CryptoHash,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(PartialEq, Clone, Default, Debug, BorshSerialize, BorshDeserialize, Eq)]
pub struct ExecutionOutcomeWithId {
    /// The transaction hash or the receipt ID.
    pub id: CryptoHash,
    /// Should be the latest field since contains unparsable by light client ExecutionStatus::Failure
    pub outcome: ExecutionOutcome,
}

impl ExecutionOutcomeWithId {
    pub fn to_hashes(&self) -> Vec<CryptoHash> {
        let mut result = vec![self.id];
        result.extend(self.outcome.to_hashes());
        result
    }
}

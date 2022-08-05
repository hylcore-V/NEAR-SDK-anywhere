use {
    crate::errors::TxExecutionError,
    borsh::{BorshDeserialize, BorshSerialize},
    core::fmt,
    near_primitives_core::{hash::CryptoHash, logging},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub enum ExecutionStatus {
    /// The execution is pending or unknown.
    Unknown,
    /// The execution has failed with the given execution error.
    Failure(TxExecutionError),
    /// The final action succeeded and returned some value or an empty vec.
    SuccessValue(Vec<u8>),
    /// The final action of the receipt returned a promise or the signed transaction was converted
    /// to a receipt. Contains the receipt_id of the generated receipt.
    SuccessReceiptId(CryptoHash),
}

impl fmt::Debug for ExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionStatus::Unknown => f.write_str("Unknown"),
            ExecutionStatus::Failure(e) => f.write_fmt(format_args!("Failure({})", e)),
            ExecutionStatus::SuccessValue(v) => {
                f.write_fmt(format_args!("SuccessValue({})", logging::pretty_utf8(v)))
            }
            ExecutionStatus::SuccessReceiptId(receipt_id) => {
                f.write_fmt(format_args!("SuccessReceiptId({})", receipt_id))
            }
        }
    }
}

impl Default for ExecutionStatus {
    fn default() -> Self {
        ExecutionStatus::Unknown
    }
}

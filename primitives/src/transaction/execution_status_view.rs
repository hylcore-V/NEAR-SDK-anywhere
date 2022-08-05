use {
    super::ExecutionStatus,
    crate::{
        errors::TxExecutionError,
        hash::CryptoHash,
        logging,
        serialize::{from_base64, to_base64},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    core::fmt,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ExecutionStatusView {
    /// The execution is pending or unknown.
    Unknown,
    /// The execution has failed.
    Failure(TxExecutionError),
    /// The final action succeeded and returned some value or an empty vec encoded in base64.
    SuccessValue(String),
    /// The final action of the receipt returned a promise or the signed transaction was converted
    /// to a receipt. Contains the receipt_id of the generated receipt.
    SuccessReceiptId(CryptoHash),
}

impl fmt::Debug for ExecutionStatusView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionStatusView::Unknown => f.write_str("Unknown"),
            ExecutionStatusView::Failure(e) => f.write_fmt(format_args!("Failure({:?})", e)),
            ExecutionStatusView::SuccessValue(v) => f.write_fmt(format_args!(
                "SuccessValue({})",
                logging::pretty_utf8(&from_base64(v).unwrap())
            )),
            ExecutionStatusView::SuccessReceiptId(receipt_id) => {
                f.write_fmt(format_args!("SuccessReceiptId({})", receipt_id))
            }
        }
    }
}

impl From<ExecutionStatus> for ExecutionStatusView {
    fn from(outcome: ExecutionStatus) -> Self {
        match outcome {
            ExecutionStatus::Unknown => ExecutionStatusView::Unknown,
            ExecutionStatus::Failure(e) => ExecutionStatusView::Failure(e),
            ExecutionStatus::SuccessValue(v) => ExecutionStatusView::SuccessValue(to_base64(&v)),
            ExecutionStatus::SuccessReceiptId(receipt_id) => {
                ExecutionStatusView::SuccessReceiptId(receipt_id)
            }
        }
    }
}

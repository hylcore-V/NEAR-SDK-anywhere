use {
    crate::{errors::TxExecutionError, logging, serialize::from_base64},
    borsh::{BorshDeserialize, BorshSerialize},
    core::fmt,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum FinalExecutionStatus {
    /// The execution has not yet started.
    NotStarted,
    /// The execution has started and still going.
    Started,
    /// The execution has failed with the given error.
    Failure(TxExecutionError),
    /// The execution has succeeded and returned some value or an empty vec encoded in base64.
    SuccessValue(String),
}

impl fmt::Debug for FinalExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FinalExecutionStatus::NotStarted => f.write_str("NotStarted"),
            FinalExecutionStatus::Started => f.write_str("Started"),
            FinalExecutionStatus::Failure(e) => f.write_fmt(format_args!("Failure({:?})", e)),
            FinalExecutionStatus::SuccessValue(v) => f.write_fmt(format_args!(
                "SuccessValue({})",
                logging::pretty_utf8(&from_base64(v).unwrap())
            )),
        }
    }
}

impl Default for FinalExecutionStatus {
    fn default() -> Self {
        FinalExecutionStatus::NotStarted
    }
}

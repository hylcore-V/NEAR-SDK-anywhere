use {
    super::InvalidTxError,
    crate::errors::ActionError,
    borsh::{BorshDeserialize, BorshSerialize},
    std::fmt::Display,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TxExecutionError {
    /// An error happened during Action execution
    ActionError(ActionError),
    /// An error happened during Transaction execution
    InvalidTxError(InvalidTxError),
}

impl std::error::Error for TxExecutionError {}

impl Display for TxExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TxExecutionError::ActionError(e) => write!(f, "{}", e),
            TxExecutionError::InvalidTxError(e) => write!(f, "{}", e),
        }
    }
}

impl From<ActionError> for TxExecutionError {
    fn from(error: ActionError) -> Self {
        TxExecutionError::ActionError(error)
    }
}

impl From<InvalidTxError> for TxExecutionError {
    fn from(error: InvalidTxError) -> Self {
        TxExecutionError::InvalidTxError(error)
    }
}

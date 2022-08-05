use {
    super::ActionErrorKind,
    borsh::{BorshDeserialize, BorshSerialize},
    std::fmt::Display,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ActionError {
    /// Index of the failed action in the transaction.
    /// Action index is not defined if ActionError.kind is `ActionErrorKind::LackBalanceForState`
    pub index: Option<u64>,
    /// The kind of ActionError happened
    pub kind: ActionErrorKind,
}

impl From<ActionErrorKind> for ActionError {
    fn from(e: ActionErrorKind) -> ActionError {
        ActionError {
            index: None,
            kind: e,
        }
    }
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Action #{}: {}",
            self.index.unwrap_or_default(),
            self.kind
        )
    }
}

impl std::error::Error for ActionError {}

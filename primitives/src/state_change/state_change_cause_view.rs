use {super::StateChangeCause, crate::hash::CryptoHash};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum StateChangeCauseView {
    NotWritableToDisk,
    InitialState,
    TransactionProcessing { tx_hash: CryptoHash },
    ActionReceiptProcessingStarted { receipt_hash: CryptoHash },
    ActionReceiptGasReward { receipt_hash: CryptoHash },
    ReceiptProcessing { receipt_hash: CryptoHash },
    PostponedReceipt { receipt_hash: CryptoHash },
    UpdatedDelayedReceipts,
    ValidatorAccountsUpdate,
    Migration,
    Resharding,
}

impl From<StateChangeCause> for StateChangeCauseView {
    fn from(state_change_cause: StateChangeCause) -> Self {
        match state_change_cause {
            StateChangeCause::NotWritableToDisk => Self::NotWritableToDisk,
            StateChangeCause::InitialState => Self::InitialState,
            StateChangeCause::TransactionProcessing { tx_hash } => {
                Self::TransactionProcessing { tx_hash }
            }
            StateChangeCause::ActionReceiptProcessingStarted { receipt_hash } => {
                Self::ActionReceiptProcessingStarted { receipt_hash }
            }
            StateChangeCause::ActionReceiptGasReward { receipt_hash } => {
                Self::ActionReceiptGasReward { receipt_hash }
            }
            StateChangeCause::ReceiptProcessing { receipt_hash } => {
                Self::ReceiptProcessing { receipt_hash }
            }
            StateChangeCause::PostponedReceipt { receipt_hash } => {
                Self::PostponedReceipt { receipt_hash }
            }
            StateChangeCause::UpdatedDelayedReceipts => Self::UpdatedDelayedReceipts,
            StateChangeCause::ValidatorAccountsUpdate => Self::ValidatorAccountsUpdate,
            StateChangeCause::Migration => Self::Migration,
            StateChangeCause::Resharding => Self::Resharding,
        }
    }
}

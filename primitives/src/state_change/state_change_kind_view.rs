use {super::StateChangeKind, crate::types::AccountId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum StateChangeKindView {
    AccountTouched { account_id: AccountId },
    AccessKeyTouched { account_id: AccountId },
    DataTouched { account_id: AccountId },
    ContractCodeTouched { account_id: AccountId },
}

impl From<StateChangeKind> for StateChangeKindView {
    fn from(state_change_kind: StateChangeKind) -> Self {
        match state_change_kind {
            StateChangeKind::AccountTouched { account_id } => Self::AccountTouched { account_id },
            StateChangeKind::AccessKeyTouched { account_id } => {
                Self::AccessKeyTouched { account_id }
            }
            StateChangeKind::DataTouched { account_id } => Self::DataTouched { account_id },
            StateChangeKind::ContractCodeTouched { account_id } => {
                Self::ContractCodeTouched { account_id }
            }
        }
    }
}

pub type StateChangesKindsView = Vec<StateChangeKindView>;

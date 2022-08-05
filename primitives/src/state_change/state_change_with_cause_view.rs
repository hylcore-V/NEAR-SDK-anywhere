use super::{StateChangeCauseView, StateChangeValueView, StateChangeWithCause};

#[derive(Debug, Serialize, Deserialize)]
pub struct StateChangeWithCauseView {
    pub cause: StateChangeCauseView,
    #[serde(flatten)]
    pub value: StateChangeValueView,
}

impl From<StateChangeWithCause> for StateChangeWithCauseView {
    fn from(state_change_with_cause: StateChangeWithCause) -> Self {
        let StateChangeWithCause { cause, value } = state_change_with_cause;
        Self {
            cause: cause.into(),
            value: value.into(),
        }
    }
}

pub type StateChangesView = Vec<StateChangeWithCauseView>;

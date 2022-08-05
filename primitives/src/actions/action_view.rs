use {
    super::{
        Action,
        AddKeyAction,
        CreateAccountAction,
        DeleteAccountAction,
        DeleteKeyAction,
        DeployContractAction,
        FunctionCallAction,
        StakeAction,
        TransferAction,
    },
    crate::{
        access_key::AccessKeyView,
        hash::hash,
        serialize::{from_base64, to_base64, u128_dec_format},
        types::{AccountId, Balance, Gas},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};

#[derive(Serialize, Deserialize, Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum ActionView {
    CreateAccount,
    DeployContract {
        code: String,
    },
    FunctionCall {
        method_name: String,
        args: String,
        gas: Gas,
        #[serde(with = "u128_dec_format")]
        deposit: Balance,
    },
    Transfer {
        #[serde(with = "u128_dec_format")]
        deposit: Balance,
    },
    Stake {
        #[serde(with = "u128_dec_format")]
        stake: Balance,
        public_key: PublicKey,
    },
    AddKey {
        public_key: PublicKey,
        access_key: AccessKeyView,
    },
    DeleteKey {
        public_key: PublicKey,
    },
    DeleteAccount {
        beneficiary_id: AccountId,
    },
}

impl From<Action> for ActionView {
    fn from(action: Action) -> Self {
        match action {
            Action::CreateAccount(_) => ActionView::CreateAccount,
            Action::DeployContract(action) => ActionView::DeployContract {
                code: to_base64(&hash(&action.code)),
            },
            Action::FunctionCall(action) => ActionView::FunctionCall {
                method_name: action.method_name,
                args: to_base64(&action.args),
                gas: action.gas,
                deposit: action.deposit,
            },
            Action::Transfer(action) => ActionView::Transfer {
                deposit: action.deposit,
            },
            Action::Stake(action) => ActionView::Stake {
                stake: action.stake,
                public_key: action.public_key,
            },
            Action::AddKey(action) => ActionView::AddKey {
                public_key: action.public_key,
                access_key: action.access_key.into(),
            },
            Action::DeleteKey(action) => ActionView::DeleteKey {
                public_key: action.public_key,
            },
            Action::DeleteAccount(action) => ActionView::DeleteAccount {
                beneficiary_id: action.beneficiary_id,
            },
        }
    }
}

impl TryFrom<ActionView> for Action {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(action_view: ActionView) -> Result<Self, Self::Error> {
        Ok(match action_view {
            ActionView::CreateAccount => Action::CreateAccount(CreateAccountAction {}),
            ActionView::DeployContract { code } => Action::DeployContract(DeployContractAction {
                code: from_base64(&code)?,
            }),
            ActionView::FunctionCall {
                method_name,
                args,
                gas,
                deposit,
            } => Action::FunctionCall(FunctionCallAction {
                method_name,
                args: from_base64(&args)?,
                gas,
                deposit,
            }),
            ActionView::Transfer { deposit } => Action::Transfer(TransferAction { deposit }),
            ActionView::Stake { stake, public_key } => {
                Action::Stake(StakeAction { stake, public_key })
            }
            ActionView::AddKey {
                public_key,
                access_key,
            } => Action::AddKey(AddKeyAction {
                public_key,
                access_key: access_key.into(),
            }),
            ActionView::DeleteKey { public_key } => {
                Action::DeleteKey(DeleteKeyAction { public_key })
            }
            ActionView::DeleteAccount { beneficiary_id } => {
                Action::DeleteAccount(DeleteAccountAction { beneficiary_id })
            }
        })
    }
}

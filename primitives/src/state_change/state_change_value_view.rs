use {
    super::StateChangeValue,
    crate::{
        access_key::AccessKeyView,
        account::AccountView,
        serialize::base64_format,
        types::{AccountId, StoreKey, StoreValue},
    },
    near_anywhere_crypto::PublicKey,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "change")]
pub enum StateChangeValueView {
    AccountUpdate {
        account_id: AccountId,
        #[serde(flatten)]
        account: AccountView,
    },
    AccountDeletion {
        account_id: AccountId,
    },
    AccessKeyUpdate {
        account_id: AccountId,
        public_key: PublicKey,
        access_key: AccessKeyView,
    },
    AccessKeyDeletion {
        account_id: AccountId,
        public_key: PublicKey,
    },
    DataUpdate {
        account_id: AccountId,
        #[serde(rename = "key_base64", with = "base64_format")]
        key: StoreKey,
        #[serde(rename = "value_base64", with = "base64_format")]
        value: StoreValue,
    },
    DataDeletion {
        account_id: AccountId,
        #[serde(rename = "key_base64", with = "base64_format")]
        key: StoreKey,
    },
    ContractCodeUpdate {
        account_id: AccountId,
        #[serde(rename = "code_base64", with = "base64_format")]
        code: Vec<u8>,
    },
    ContractCodeDeletion {
        account_id: AccountId,
    },
}

impl From<StateChangeValue> for StateChangeValueView {
    fn from(state_change: StateChangeValue) -> Self {
        match state_change {
            StateChangeValue::AccountUpdate {
                account_id,
                account,
            } => Self::AccountUpdate {
                account_id,
                account: account.into(),
            },
            StateChangeValue::AccountDeletion { account_id } => {
                Self::AccountDeletion { account_id }
            }
            StateChangeValue::AccessKeyUpdate {
                account_id,
                public_key,
                access_key,
            } => Self::AccessKeyUpdate {
                account_id,
                public_key,
                access_key: access_key.into(),
            },
            StateChangeValue::AccessKeyDeletion {
                account_id,
                public_key,
            } => Self::AccessKeyDeletion {
                account_id,
                public_key,
            },
            StateChangeValue::DataUpdate {
                account_id,
                key,
                value,
            } => Self::DataUpdate {
                account_id,
                key,
                value,
            },
            StateChangeValue::DataDeletion { account_id, key } => {
                Self::DataDeletion { account_id, key }
            }
            StateChangeValue::ContractCodeUpdate { account_id, code } => {
                Self::ContractCodeUpdate { account_id, code }
            }
            StateChangeValue::ContractCodeDeletion { account_id } => {
                Self::ContractCodeDeletion { account_id }
            }
        }
    }
}

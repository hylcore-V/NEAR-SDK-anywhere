use {
    crate::{
        account::{AccessKey, Account},
        types::{AccountId, StoreKey, StoreValue},
    },
    near_anywhere_crypto::PublicKey,
};

#[derive(Debug)]
pub enum StateChangeValue {
    AccountUpdate {
        account_id: AccountId,
        account: Account,
    },
    AccountDeletion {
        account_id: AccountId,
    },
    AccessKeyUpdate {
        account_id: AccountId,
        public_key: PublicKey,
        access_key: AccessKey,
    },
    AccessKeyDeletion {
        account_id: AccountId,
        public_key: PublicKey,
    },
    DataUpdate {
        account_id: AccountId,
        key: StoreKey,
        value: StoreValue,
    },
    DataDeletion {
        account_id: AccountId,
        key: StoreKey,
    },
    ContractCodeUpdate {
        account_id: AccountId,
        code: Vec<u8>,
    },
    ContractCodeDeletion {
        account_id: AccountId,
    },
}

impl StateChangeValue {
    pub fn affected_account_id(&self) -> &AccountId {
        match &self {
            StateChangeValue::AccountUpdate { account_id, .. }
            | StateChangeValue::AccountDeletion { account_id }
            | StateChangeValue::AccessKeyUpdate { account_id, .. }
            | StateChangeValue::AccessKeyDeletion { account_id, .. }
            | StateChangeValue::DataUpdate { account_id, .. }
            | StateChangeValue::DataDeletion { account_id, .. }
            | StateChangeValue::ContractCodeUpdate { account_id, .. }
            | StateChangeValue::ContractCodeDeletion { account_id } => account_id,
        }
    }
}

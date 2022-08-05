use crate::{
    account::AccountWithPublicKey,
    types::{AccountId, StoreKey},
};

#[derive(Debug)]
pub enum StateChangesRequest {
    AccountChanges {
        account_ids: Vec<AccountId>,
    },
    SingleAccessKeyChanges {
        keys: Vec<AccountWithPublicKey>,
    },
    AllAccessKeyChanges {
        account_ids: Vec<AccountId>,
    },
    ContractCodeChanges {
        account_ids: Vec<AccountId>,
    },
    DataChanges {
        account_ids: Vec<AccountId>,
        key_prefix: StoreKey,
    },
}

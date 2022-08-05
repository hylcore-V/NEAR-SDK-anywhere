use crate::primitives::{
    account::{AccessKey, AccessKeyPermission, FunctionCallPermission},
    types::Balance,
};

mod account;
mod account_authorized_app;
mod account_balance;

pub use {
    account::Account,
    account_authorized_app::AccountAuthorizedApp,
    account_balance::AccountBalance,
};

pub fn full_access_key() -> AccessKey {
    AccessKey {
        nonce: 0,
        permission: AccessKeyPermission::FullAccess,
    }
}

pub fn funcation_call_access_key(
    receiver_id: String,
    method_names: Vec<String>,
    allowance: Option<Balance>,
) -> AccessKey {
    AccessKey {
        nonce: 0,
        permission: AccessKeyPermission::FunctionCall(FunctionCallPermission {
            allowance,
            receiver_id,
            method_names,
        }),
    }
}

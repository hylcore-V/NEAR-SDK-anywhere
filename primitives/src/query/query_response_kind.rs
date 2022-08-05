use {
    crate::{
        access_key::{AccessKeyList, AccessKeyView},
        account::AccountView,
        contract::ContractCodeView,
        transaction::{CallResult, ViewStateResult},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum QueryResponseKind {
    ViewAccount(AccountView),
    ViewCode(ContractCodeView),
    ViewState(ViewStateResult),
    CallResult(CallResult),
    AccessKey(AccessKeyView),
    AccessKeyList(AccessKeyList),
}

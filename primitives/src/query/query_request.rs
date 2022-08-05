use {
    super::FunctionArgs,
    crate::{
        serialize::base64_format,
        types::{AccountId, StoreKey},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "request_type", rename_all = "snake_case")]
pub enum QueryRequest {
    ViewAccount {
        account_id: AccountId,
    },

    ViewCode {
        account_id: AccountId,
    },

    ViewState {
        account_id: AccountId,
        #[serde(rename = "prefix_base64", with = "base64_format")]
        prefix: StoreKey,
    },
    ViewAccessKey {
        account_id: AccountId,
        public_key: PublicKey,
    },
    ViewAccessKeyList {
        account_id: AccountId,
    },
    CallFunction {
        account_id: AccountId,
        method_name: String,
        #[serde(rename = "args_base64", with = "base64_format")]
        args: FunctionArgs,
    },
}

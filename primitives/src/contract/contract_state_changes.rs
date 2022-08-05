use {
    crate::{
        serialize::base64_format,
        types::{AccountId, StoreKey, StoreValue},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct ContractStateChanges {
    account_id: AccountId,
    #[serde(alias = "key_base64", with = "base64_format")]
    key: StoreKey,
    #[serde(alias = "value_base64", with = "base64_format")]
    value: StoreValue,
}

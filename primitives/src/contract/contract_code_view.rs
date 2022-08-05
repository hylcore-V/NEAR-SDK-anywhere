use {
    crate::{hash::CryptoHash, serialize::base64_format},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ContractCodeView {
    #[serde(rename = "code_base64", with = "base64_format")]
    pub code: Vec<u8>,
    pub hash: CryptoHash,
}

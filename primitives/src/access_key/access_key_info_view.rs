use {
    super::AccessKeyView,
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AccessKeyInfoView {
    pub public_key: PublicKey,
    pub access_key: AccessKeyView,
}

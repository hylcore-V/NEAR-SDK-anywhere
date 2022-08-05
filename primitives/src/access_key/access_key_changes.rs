use {
    crate::{account::AccessKey, types::AccountId},
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AccessKeyChanges {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub access_key: AccessKey,
}

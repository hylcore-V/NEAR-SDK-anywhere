use {
    super::Action,
    crate::account::AccessKey,
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AddKeyAction {
    /// A public key which will be associated with an access_key
    pub public_key: PublicKey,
    /// An access key with the permission
    pub access_key: AccessKey,
}

impl From<AddKeyAction> for Action {
    fn from(add_key_action: AddKeyAction) -> Self {
        Self::AddKey(add_key_action)
    }
}

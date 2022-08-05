use {
    super::AccessKeyPermissionView,
    crate::{account::AccessKey, types::Nonce},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct AccessKeyView {
    pub nonce: Nonce,
    pub permission: AccessKeyPermissionView,
}

impl From<AccessKey> for AccessKeyView {
    fn from(access_key: AccessKey) -> Self {
        Self {
            nonce: access_key.nonce,
            permission: access_key.permission.into(),
        }
    }
}

impl From<AccessKeyView> for AccessKey {
    fn from(view: AccessKeyView) -> Self {
        Self {
            nonce: view.nonce,
            permission: view.permission.into(),
        }
    }
}

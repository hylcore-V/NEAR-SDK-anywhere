use {
    super::AccessKeyInfoView,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AccessKeyList {
    pub keys: Vec<AccessKeyInfoView>,
}

impl FromIterator<AccessKeyInfoView> for AccessKeyList {
    fn from_iter<I: IntoIterator<Item = AccessKeyInfoView>>(iter: I) -> Self {
        Self {
            keys: iter.into_iter().collect(),
        }
    }
}

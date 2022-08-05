use {
    crate::hash::CryptoHash,
    borsh::{BorshDeserialize, BorshSerialize},
    derive_more::AsRef as DeriveAsRef,
};

#[derive(
    Debug,
    Clone,
    Default,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    DeriveAsRef,
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
)]
#[as_ref(forward)]
pub struct EpochId(pub CryptoHash);

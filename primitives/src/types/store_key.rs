use {
    borsh::{BorshDeserialize, BorshSerialize},
    derive_more::{AsRef as DeriveAsRef, From as DeriveFrom},
};

#[derive(
    Debug, Clone, PartialEq, Eq, DeriveAsRef, DeriveFrom, BorshSerialize, BorshDeserialize,
)]
#[as_ref(forward)]
pub struct StoreKey(Vec<u8>);

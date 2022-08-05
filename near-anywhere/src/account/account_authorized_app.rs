use crate::{
    crypto::PublicKey,
    primitives::types::{AccountId, Balance},
};

#[derive(Clone)]
pub struct AccountAuthorizedApp {
    pub contract_id: AccountId,
    pub amount: Balance,
    pub public_key: PublicKey,
}

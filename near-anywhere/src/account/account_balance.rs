use crate::primitives::types::Balance;

#[derive(Clone)]
pub struct AccountBalance {
    pub total: Balance,
    pub state_staked: Balance,
    pub staked: Balance,
    pub available: Balance,
}

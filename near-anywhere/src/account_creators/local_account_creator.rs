use crate::{account::Account, crypto::PublicKey, primitives::types::Balance};

pub struct LocalAccountCreator {
    master_account: Account,
    initial_balance: Balance,
}

impl LocalAccountCreator {
    pub fn new(master_account: Account, initial_balance: Balance) -> Self {
        Self {
            master_account,
            initial_balance,
        }
    }

    pub async fn create_account(&mut self, account_id: &str, public_key: PublicKey) {
        self.master_account
            .create_account(account_id, public_key, self.initial_balance.clone())
            .await
            .unwrap();
    }
}

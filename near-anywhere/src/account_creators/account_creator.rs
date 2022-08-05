use {
    super::{LocalAccountCreator, UrlAccountCreator},
    crate::crypto::PublicKey,
};

pub enum AccountCreator {
    LocalAccountCreator(LocalAccountCreator),
    UrlAccountCreator(UrlAccountCreator),
}

impl AccountCreator {
    pub async fn create_account(&mut self, account_id: &str, public_key: PublicKey) {
        match self {
            AccountCreator::LocalAccountCreator(account_creator) => {
                account_creator.create_account(account_id, public_key).await;
            }
            AccountCreator::UrlAccountCreator(account_creator) => {
                account_creator.create_account(account_id, public_key).await;
            }
        }
    }
}

use {
    crate::{
        account::Account,
        hash::CryptoHash,
        serialize::u128_dec_format,
        types::{Balance, BlockHeight, StorageUsage},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct AccountView {
    #[serde(with = "u128_dec_format")]
    pub amount: Balance,
    #[serde(with = "u128_dec_format")]
    pub locked: Balance,
    pub code_hash: CryptoHash,
    pub storage_usage: StorageUsage,
    /// TODO(2271): deprecated.
    #[serde(default)]
    pub storage_paid_at: BlockHeight,
}

impl From<&Account> for AccountView {
    fn from(account: &Account) -> Self {
        AccountView {
            amount: account.amount(),
            locked: account.locked(),
            code_hash: account.code_hash(),
            storage_usage: account.storage_usage(),
            storage_paid_at: 0,
        }
    }
}

impl From<Account> for AccountView {
    fn from(account: Account) -> Self {
        (&account).into()
    }
}

impl From<&AccountView> for Account {
    fn from(view: &AccountView) -> Self {
        Account::new(view.amount, view.locked, view.code_hash, view.storage_usage)
    }
}

impl From<AccountView> for Account {
    fn from(view: AccountView) -> Self {
        (&view).into()
    }
}

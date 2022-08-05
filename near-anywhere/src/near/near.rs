use {
    super::{Connection, NearConfig},
    crate::account::Account,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Near {
    pub config: NearConfig,
    pub connection: Connection,
}

impl Near {
    pub fn new(config: NearConfig) -> Self {
        let connection = Connection::try_from(config.clone()).unwrap();

        Self { config, connection }
    }

    pub fn account(&self, account_id: &str) -> Account {
        Account::new(self.connection.clone(), account_id.parse().unwrap())
    }
}

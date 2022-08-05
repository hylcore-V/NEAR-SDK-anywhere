use {
    super::AccountCreationConfig,
    crate::{
        config::VMConfig,
        runtime::fees::RuntimeFeesConfig,
        serialize::u128_dec_format,
        types::Balance,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// Amount of yN per byte required to have on the account.  See
    /// <https://nomicon.io/Economics/README.html#state-stake> for details.
    #[serde(with = "u128_dec_format")]
    pub storage_amount_per_byte: Balance,
    /// Costs of different actions that need to be performed when sending and processing transaction
    /// and receipts.
    pub transaction_costs: RuntimeFeesConfig,
    /// Config of wasm operations.
    pub wasm_config: VMConfig,
    /// Config that defines rules for account creation.
    pub account_creation_config: AccountCreationConfig,
}

use {
    super::{
        AddKeyAction,
        CreateAccountAction,
        DeleteAccountAction,
        DeleteKeyAction,
        DeployContractAction,
        FunctionCallAction,
        StakeAction,
        TransferAction,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_primitives_core::types::{Balance, Gas},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Action {
    /// Create an (sub)account using a transaction `receiver_id` as an ID for
    /// a new account ID must pass validation rules described here
    /// <http://nomicon.io/Primitives/Account.html>.
    CreateAccount(CreateAccountAction),
    /// Sets a Wasm code to a receiver_id
    DeployContract(DeployContractAction),
    FunctionCall(FunctionCallAction),
    Transfer(TransferAction),
    Stake(StakeAction),
    AddKey(AddKeyAction),
    DeleteKey(DeleteKeyAction),
    DeleteAccount(DeleteAccountAction),
}

impl Action {
    pub fn get_prepaid_gas(&self) -> Gas {
        match self {
            Action::FunctionCall(a) => a.gas,
            _ => 0,
        }
    }
    pub fn get_deposit_balance(&self) -> Balance {
        match self {
            Action::FunctionCall(a) => a.deposit,
            Action::Transfer(a) => a.deposit,
            _ => 0,
        }
    }
}

use {
    super::ReceiptValidationError,
    crate::serialize::u128_dec_format,
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
    near_primitives_core::types::{AccountId, Balance},
    std::fmt::Display,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ActionErrorKind {
    /// Happens when CreateAccount action tries to create an account with account_id which is already exists in the storage
    AccountAlreadyExists { account_id: AccountId },
    /// Happens when TX receiver_id doesn't exist (but action is not Action::CreateAccount)
    AccountDoesNotExist { account_id: AccountId },
    /// A top-level account ID can only be created by registrar.
    CreateAccountOnlyByRegistrar {
        account_id: AccountId,
        registrar_account_id: AccountId,
        predecessor_id: AccountId,
    },
    /// A newly created account must be under a namespace of the creator account
    CreateAccountNotAllowed {
        account_id: AccountId,
        predecessor_id: AccountId,
    },
    /// Administrative actions like `DeployContract`, `Stake`, `AddKey`, `DeleteKey`. can be proceed only if sender=receiver
    /// or the first TX action is a `CreateAccount` action
    ActorNoPermission {
        account_id: AccountId,
        actor_id: AccountId,
    },
    /// Account tries to remove an access key that doesn't exist
    DeleteKeyDoesNotExist {
        account_id: AccountId,
        public_key: PublicKey,
    },
    /// The public key is already used for an existing access key
    AddKeyAlreadyExists {
        account_id: AccountId,
        public_key: PublicKey,
    },
    /// Account is staking and can not be deleted
    DeleteAccountStaking { account_id: AccountId },
    /// ActionReceipt can't be completed, because the remaining balance will not be enough to cover storage.
    LackBalanceForState {
        /// An account which needs balance
        account_id: AccountId,
        /// Balance required to complete an action.
        #[serde(with = "u128_dec_format")]
        amount: Balance,
    },
    /// Account is not yet staked, but tries to unstake
    TriesToUnstake { account_id: AccountId },
    /// The account doesn't have enough balance to increase the stake.
    TriesToStake {
        account_id: AccountId,
        #[serde(with = "u128_dec_format")]
        stake: Balance,
        #[serde(with = "u128_dec_format")]
        locked: Balance,
        #[serde(with = "u128_dec_format")]
        balance: Balance,
    },
    InsufficientStake {
        account_id: AccountId,
        #[serde(with = "u128_dec_format")]
        stake: Balance,
        #[serde(with = "u128_dec_format")]
        minimum_stake: Balance,
    },
    /// An error occurred during a `FunctionCall` Action, parameter is debug message.
    FunctionCallError(String),
    /// Error occurs when a new `ActionReceipt` created by the `FunctionCall` action fails
    /// receipt validation.
    NewReceiptValidationError(ReceiptValidationError),
    /// Error occurs when a `CreateAccount` action is called on hex-characters
    /// account of length 64.  See implicit account creation NEP:
    /// <https://github.com/nearprotocol/NEPs/pull/71>.
    OnlyImplicitAccountCreationAllowed { account_id: AccountId },
    /// Delete account whose state is large is temporarily banned.
    DeleteAccountWithLargeState { account_id: AccountId },
}

impl Display for ActionErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ActionErrorKind::AccountAlreadyExists { account_id } => {
                write!(f, "Can't create a new account {:?}, because it already exists", account_id)
            }
            ActionErrorKind::AccountDoesNotExist { account_id } => write!(
                f,
                "Can't complete the action because account {:?} doesn't exist",
                account_id
            ),
            ActionErrorKind::ActorNoPermission { actor_id, account_id } => write!(
                f,
                "Actor {:?} doesn't have permission to account {:?} to complete the action",
                actor_id, account_id
            ),
            ActionErrorKind::LackBalanceForState { account_id, amount } => write!(
                f,
                "The account {} wouldn't have enough balance to cover storage, required to have {} yoctoNEAR more",
                account_id, amount
            ),
            ActionErrorKind::TriesToUnstake { account_id } => {
                write!(f, "Account {:?} is not yet staked, but tries to unstake", account_id)
            }
            ActionErrorKind::TriesToStake { account_id, stake, locked, balance } => write!(
                f,
                "Account {:?} tries to stake {}, but has staked {} and only has {}",
                account_id, stake, locked, balance
            ),
            ActionErrorKind::CreateAccountOnlyByRegistrar { account_id, registrar_account_id, predecessor_id } => write!(
                f,
                "A top-level account ID {:?} can't be created by {:?}, short top-level account IDs can only be created by {:?}",
                account_id, predecessor_id, registrar_account_id,
            ),
            ActionErrorKind::CreateAccountNotAllowed { account_id, predecessor_id } => write!(
                f,
                "A sub-account ID {:?} can't be created by account {:?}",
                account_id, predecessor_id,
            ),
            ActionErrorKind::DeleteKeyDoesNotExist { account_id, .. } => write!(
                f,
                "Account {:?} tries to remove an access key that doesn't exist",
                account_id
            ),
            ActionErrorKind::AddKeyAlreadyExists { public_key, .. } => write!(
                f,
                "The public key {:?} is already used for an existing access key",
                public_key
            ),
            ActionErrorKind::DeleteAccountStaking { account_id } => {
                write!(f, "Account {:?} is staking and can not be deleted", account_id)
            }
            ActionErrorKind::FunctionCallError(s) => write!(f, "{:?}", s),
            ActionErrorKind::NewReceiptValidationError(e) => {
                write!(f, "An new action receipt created during a FunctionCall is not valid: {}", e)
            }
            ActionErrorKind::InsufficientStake { account_id, stake, minimum_stake } => write!(f, "Account {} tries to stake {} but minimum required stake is {}", account_id, stake, minimum_stake),
            ActionErrorKind::OnlyImplicitAccountCreationAllowed { account_id } => write!(f, "CreateAccount action is called on hex-characters account of length 64 {}", account_id),
            ActionErrorKind::DeleteAccountWithLargeState { account_id } => write!(f, "The state of account {} is too large and therefore cannot be deleted", account_id),
        }
    }
}

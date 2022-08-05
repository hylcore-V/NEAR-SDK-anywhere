use {
    crate::serialize::u128_dec_format,
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
    near_primitives_core::types::{AccountId, Balance},
    std::fmt::Display,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum InvalidAccessKeyError {
    /// The access key identified by the `public_key` doesn't exist for the account
    AccessKeyNotFound {
        account_id: AccountId,
        public_key: PublicKey,
    },
    /// Transaction `receiver_id` doesn't match the access key receiver_id
    ReceiverMismatch {
        tx_receiver: AccountId,
        ak_receiver: String,
    },
    /// Transaction method name isn't allowed by the access key
    MethodNameMismatch { method_name: String },
    /// Transaction requires a full permission access key.
    RequiresFullAccess,
    /// Access Key does not have enough allowance to cover transaction cost
    NotEnoughAllowance {
        account_id: AccountId,
        public_key: PublicKey,
        #[serde(with = "u128_dec_format")]
        allowance: Balance,
        #[serde(with = "u128_dec_format")]
        cost: Balance,
    },
    /// Having a deposit with a function call action is not allowed with a function call access key.
    DepositWithFunctionCall,
}

impl Display for InvalidAccessKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InvalidAccessKeyError::AccessKeyNotFound {
                account_id,
                public_key,
            } => write!(
                f,
                "Signer {:?} doesn't have access key with the given public_key {}",
                account_id, public_key
            ),
            InvalidAccessKeyError::ReceiverMismatch {
                tx_receiver,
                ak_receiver,
            } => write!(
                f,
                "Transaction receiver_id {:?} doesn't match the access key receiver_id {:?}",
                tx_receiver, ak_receiver
            ),
            InvalidAccessKeyError::MethodNameMismatch { method_name } => write!(
                f,
                "Transaction method name {:?} isn't allowed by the access key",
                method_name
            ),
            InvalidAccessKeyError::RequiresFullAccess => {
                write!(f, "Invalid access key type. Full-access keys are required for transactions that have multiple or non-function-call actions")
            }
            InvalidAccessKeyError::NotEnoughAllowance {
                account_id,
                public_key,
                allowance,
                cost,
            } => write!(
                f,
                "Access Key {:?}:{} does not have enough balance {} for transaction costing {}",
                account_id, public_key, allowance, cost
            ),
            InvalidAccessKeyError::DepositWithFunctionCall => {
                write!(f, "Having a deposit with a function call action is not allowed with a function call access key.")
            }
        }
    }
}

impl std::error::Error for InvalidAccessKeyError {}

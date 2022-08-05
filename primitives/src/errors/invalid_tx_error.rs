use {
    super::{ActionsValidationError, InvalidAccessKeyError},
    crate::serialize::u128_dec_format,
    borsh::{BorshDeserialize, BorshSerialize},
    near_primitives_core::types::{AccountId, Balance, Nonce},
    std::fmt::Display,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum InvalidTxError {
    /// Happens if a wrong AccessKey used or AccessKey has not enough permissions
    InvalidAccessKeyError(InvalidAccessKeyError),
    /// TX signer_id is not a valid [`AccountId`]
    InvalidSignerId { signer_id: String },
    /// TX signer_id is not found in a storage
    SignerDoesNotExist { signer_id: AccountId },
    /// Transaction nonce must be `account[access_key].nonce + 1`.
    InvalidNonce { tx_nonce: Nonce, ak_nonce: Nonce },
    /// Transaction nonce is larger than the upper bound given by the block height
    NonceTooLarge { tx_nonce: Nonce, upper_bound: Nonce },
    /// TX receiver_id is not a valid AccountId
    InvalidReceiverId { receiver_id: String },
    /// TX signature is not valid
    InvalidSignature,
    /// Account does not have enough balance to cover TX cost
    NotEnoughBalance {
        signer_id: AccountId,
        #[serde(with = "u128_dec_format")]
        balance: Balance,
        #[serde(with = "u128_dec_format")]
        cost: Balance,
    },
    /// Signer account doesn't have enough balance after transaction.
    LackBalanceForState {
        /// An account which doesn't have enough balance to cover storage.
        signer_id: AccountId,
        /// Required balance to cover the state.
        #[serde(with = "u128_dec_format")]
        amount: Balance,
    },
    /// An integer overflow occurred during transaction cost estimation.
    CostOverflow,
    /// Transaction parent block hash doesn't belong to the current chain
    InvalidChain,
    /// Transaction has expired
    Expired,
    /// An error occurred while validating actions of a Transaction.
    ActionsValidation(ActionsValidationError),
    /// The size of serialized transaction exceeded the limit.
    TransactionSizeExceeded { size: u64, limit: u64 },
}

impl std::error::Error for InvalidTxError {}

impl Display for InvalidTxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InvalidTxError::InvalidSignerId { signer_id } => {
                write!(
                    f,
                    "Invalid signer account ID {:?} according to requirements",
                    signer_id
                )
            }
            InvalidTxError::SignerDoesNotExist { signer_id } => {
                write!(f, "Signer {:?} does not exist", signer_id)
            }
            InvalidTxError::InvalidAccessKeyError(access_key_error) => {
                Display::fmt(&access_key_error, f)
            }
            InvalidTxError::InvalidNonce { tx_nonce, ak_nonce } => write!(
                f,
                "Transaction nonce {} must be larger than nonce of the used access key {}",
                tx_nonce, ak_nonce
            ),
            InvalidTxError::InvalidReceiverId { receiver_id } => {
                write!(
                    f,
                    "Invalid receiver account ID {:?} according to requirements",
                    receiver_id
                )
            }
            InvalidTxError::InvalidSignature => {
                write!(f, "Transaction is not signed with the given public key")
            }
            InvalidTxError::NotEnoughBalance {
                signer_id,
                balance,
                cost,
            } => write!(
                f,
                "Sender {:?} does not have enough balance {} for operation costing {}",
                signer_id, balance, cost
            ),
            InvalidTxError::LackBalanceForState { signer_id, amount } => {
                write!(f, "Failed to execute, because the account {:?} wouldn't have enough balance to cover storage, required to have {} yoctoNEAR more", signer_id, amount)
            }
            InvalidTxError::CostOverflow => {
                write!(f, "Transaction gas or balance cost is too high")
            }
            InvalidTxError::InvalidChain => {
                write!(
                    f,
                    "Transaction parent block hash doesn't belong to the current chain"
                )
            }
            InvalidTxError::Expired => {
                write!(f, "Transaction has expired")
            }
            InvalidTxError::ActionsValidation(error) => {
                write!(f, "Transaction actions validation error: {}", error)
            }
            InvalidTxError::NonceTooLarge {
                tx_nonce,
                upper_bound,
            } => {
                write!(
                    f,
                    "Transaction nonce {} must be smaller than the access key nonce upper bound {}",
                    tx_nonce, upper_bound
                )
            }
            InvalidTxError::TransactionSizeExceeded { size, limit } => {
                write!(
                    f,
                    "Size of serialized transaction {} exceeded the limit {}",
                    size, limit
                )
            }
        }
    }
}

impl From<InvalidAccessKeyError> for InvalidTxError {
    fn from(error: InvalidAccessKeyError) -> Self {
        InvalidTxError::InvalidAccessKeyError(error)
    }
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "info", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcStateChangesError {
    #[error("Block not found: {error_message}")]
    UnknownBlock {
        #[serde(skip_serializing)]
        error_message: String,
    },
    #[error("There are no fully synchronized blocks yet")]
    NotSyncedYet,
    #[error("The node reached its limits. Try again later. More details: {error_message}")]
    InternalError { error_message: String },
}

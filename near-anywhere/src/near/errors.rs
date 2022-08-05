#[derive(Debug, Clone, thiserror::Error)]
pub enum ConnectionError {
    #[error("invalid params: {error_message}")]
    InvalidParams { error_message: String },
}

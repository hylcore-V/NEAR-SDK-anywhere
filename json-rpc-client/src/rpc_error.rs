use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

impl From<reqwest::Error> for RpcError {
    fn from(error: reqwest::Error) -> Self {
        RpcError {
            code: error
                .status()
                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                .as_u16() as i32,
            message: error.to_string(),
        }
    }
}

impl Display for RpcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl std::error::Error for RpcError {}

use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub id: u32,
    #[serde(rename(serialize = "jsonrpc", deserialize = "jsonrpc"))]
    pub jsonrpc_version: String,
    pub method: String,
    pub params: Option<Value>,
}

impl RpcRequest {
    pub fn new(method: &str) -> Self {
        Self {
            id: 0,
            jsonrpc_version: "2.0".to_owned(),
            method: method.to_owned(),
            params: None,
        }
    }
}

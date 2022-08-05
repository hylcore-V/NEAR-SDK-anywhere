use crate::{RpcError, RpcRequest, RpcResponse, Transport};

#[derive(Clone, Debug, PartialEq)]
pub struct RpcClient {
    transport: Transport,
}

impl RpcClient {
    pub fn new(transport: Transport) -> Self {
        Self { transport }
    }

    pub async fn send(&self, request: RpcRequest) -> Result<RpcResponse, RpcError> {
        match self.transport.clone() {
            Transport::Http(http) => http.send(&request).await,
        }
    }
}

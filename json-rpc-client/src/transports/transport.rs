use crate::{Http, RpcError, RpcRequest, RpcResponse};

#[derive(Debug, Clone, PartialEq)]
pub enum Transport {
    Http(Http),
}

impl Transport {
    pub fn http(url: &str) -> Self {
        Self::Http(Http::new(url))
    }

    pub async fn send(self, request: &RpcRequest) -> Result<RpcResponse, RpcError> {
        match self {
            Transport::Http(http) => http.send(request).await,
        }
    }
}

use crate::{RpcError, RpcRequest, RpcResponse};

#[derive(Clone)]
pub struct Http {
    client: reqwest::Client,
    headers: reqwest::header::HeaderMap,
    url: String,
}

impl Http {
    pub fn new(url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            headers: reqwest::header::HeaderMap::new(),
            url: url.to_string(),
        }
    }
}

impl Http {
    pub async fn send(self, request: &RpcRequest) -> Result<RpcResponse, RpcError> {
        let client = &self.client;
        let url = self.url.clone();
        let headers = self.headers.clone();

        let response: serde_json::Value = client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| RpcError::from(e))?
            .json()
            .await
            .map_err(|e| RpcError::from(e))?;

        match serde_json::from_value::<RpcResponse>(response.clone()) {
            Ok(response) => Ok(response),
            Err(_) => Err(serde_json::from_value::<RpcError>(response).unwrap()),
        }
    }
}

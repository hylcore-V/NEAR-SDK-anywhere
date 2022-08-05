use {
    crate::{
        block::BlockView,
        config::{RpcProtocolConfigRequest, RpcProtocolConfigResponse},
        light_client::{RpcLightClientExecutionProofRequest, RpcLightClientExecutionProofResponse},
        network::{EpochValidatorInfo, NetworkInfoView, StatusResponse},
        query::{RpcQueryRequest, RpcQueryResponse},
        sharding::{ChunkId, ChunkView},
        state_change::{
            RpcStateChangesInBlockByTypeRequest,
            RpcStateChangesInBlockByTypeResponse,
            RpcStateChangesInBlockRequest,
            RpcStateChangesInBlockResponse,
        },
        transaction::{FinalExecutionOutcomeView, SignedTransaction},
        types::{BlockId, BlockReference, GasPriceView, MaybeBlockId},
    },
    borsh::BorshSerialize,
    json_rpc_client::{RpcClient, RpcError, RpcRequest, RpcResponse, Transport},
    near_primitives_core::{
        hash::CryptoHash,
        serialize::to_base64,
        types::{AccountId, BlockHeight},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct NearRpcClient(RpcClient);

impl NearRpcClient {
    pub fn new_with_http(url: &str) -> Self {
        let transport = Transport::http(url);
        let client = RpcClient::new(transport);
        Self { 0: client }
    }

    pub fn new(transport: Transport) -> Self {
        let client = RpcClient::new(transport);
        Self { 0: client }
    }

    async fn call_method(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<RpcResponse, RpcError> {
        let mut request = RpcRequest::new(method);
        request.params = params;
        let response = self.0.send(request).await?;

        Ok(response)
    }

    pub async fn broadcast_tx_async(&self, tx: SignedTransaction) -> Result<CryptoHash, RpcError> {
        let input = tx.try_to_vec().unwrap();
        let value = to_base64(&input);
        let params = serde_json::to_value(vec![value]).unwrap();
        let response = self.call_method("broadcast_tx_async", Some(params)).await?;
        let value = serde_json::from_value(response.result).unwrap();
        let result = CryptoHash::from(value);

        Ok(result)
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: SignedTransaction,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let input = tx.try_to_vec().unwrap();
        let value = to_base64(&input);
        let params = serde_json::to_value(vec![value]).unwrap();
        let response = self
            .call_method("broadcast_tx_commit", Some(params))
            .await?;
        let value = serde_json::from_value(response.result).unwrap();
        let result = FinalExecutionOutcomeView::from(value);

        Ok(result)
    }

    pub async fn status(&self) -> Result<StatusResponse, RpcError> {
        let response = self.call_method("status", None).await?;
        let result = serde_json::from_value(response.result).unwrap();

        Ok(result)
    }

    pub async fn chunk(&self, chunk_id: ChunkId) -> Result<ChunkView, RpcError> {
        let params = serde_json::to_value(chunk_id).unwrap();
        let response = self.call_method("chunk", Some(params)).await?;
        let result = serde_json::from_value(response.result).unwrap();

        Ok(result)
    }

    pub async fn tx(
        &self,
        hash: CryptoHash,
        account_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let params = serde_json::to_value([hash.to_string(), account_id.to_string()]).unwrap();
        let response = self.call_method("tx", Some(params)).await?;
        let result = serde_json::from_value(response.result).unwrap();

        Ok(result)
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(
        &self,
        hash: CryptoHash,
        account_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let params = serde_json::to_value([hash.to_string(), account_id.to_string()]).unwrap();
        let response = self
            .call_method("EXPERIMENTAL_tx_status", Some(params))
            .await?;
        let result = serde_json::from_value(response.result).unwrap();

        Ok(result)
    }

    pub async fn gas_price(&self, height: BlockHeight) -> Result<GasPriceView, RpcError> {
        let request = BlockId::Height(height);
        let params = serde_json::to_value([request]).unwrap();
        let response = self.call_method("gas_price", Some(params)).await?;
        let result = serde_json::from_value(response.result).unwrap();

        Ok(result)
    }

    pub async fn query(&self, request: RpcQueryRequest) -> RpcQueryResponse {
        let params = serde_json::to_value(request).unwrap();
        let response = self.call_method("query", Some(params)).await.unwrap();
        let result = serde_json::from_value(response.result).unwrap();

        result
    }
    pub async fn block(&self, block_reference: BlockReference) -> Result<BlockView, RpcError> {
        let params = serde_json::to_value(block_reference).unwrap();
        let response = self.call_method("block", Some(params)).await?;
        let result = serde_json::from_value(response.result).unwrap();

        Ok(result)
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes_in_block(
        &self,
        request: RpcStateChangesInBlockRequest,
    ) -> RpcStateChangesInBlockByTypeResponse {
        let params = serde_json::to_value(request).unwrap();
        let response = self
            .call_method("EXPERIMENTAL_changes_in_block", Some(params))
            .await
            .unwrap();
        let result = response.result;

        serde_json::from_value(result).unwrap()
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: RpcStateChangesInBlockByTypeRequest,
    ) -> RpcStateChangesInBlockResponse {
        let params = serde_json::to_value(request).unwrap();
        let response = self
            .call_method("EXPERIMENTAL_changes", Some(params))
            .await
            .unwrap();
        let result = response.result;

        serde_json::from_value(result).unwrap()
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: RpcProtocolConfigRequest,
    ) -> RpcProtocolConfigResponse {
        let params = serde_json::to_value(request).unwrap();
        let response = self
            .call_method("EXPERIMENTAL_protocol_config", Some(params))
            .await
            .unwrap();
        let result = response.result;

        serde_json::from_value(result).unwrap()
    }

    pub async fn light_client_proof(
        &self,
        request: RpcLightClientExecutionProofRequest,
    ) -> RpcLightClientExecutionProofResponse {
        let params = serde_json::to_value(request).unwrap();
        let response = self
            .call_method("light_client_proof", Some(params))
            .await
            .unwrap();
        let result = response.result;

        serde_json::from_value(result).unwrap()
    }

    pub async fn validators(&self, block_id: MaybeBlockId) -> EpochValidatorInfo {
        let params = serde_json::to_value([block_id]).unwrap();
        let response = self.call_method("validators", Some(params)).await.unwrap();
        let result = response.result;

        serde_json::from_value(result).unwrap()
    }

    pub async fn network_info(&self) -> NetworkInfoView {
        let response = self.call_method("network_info", None).await.unwrap();
        let result = response.result;

        serde_json::from_value(result).unwrap()
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(&self) -> serde_json::Value {
        let response = self
            .call_method("EXPERIMENTAL_genesis_config", None)
            .await
            .unwrap();

        response.result
    }
}

use {
    super::NearRpcClient,
    crate::{
        access_key::{AccessKeyList, AccessKeyView},
        account::{AccountView, AccountWithPublicKey},
        block::BlockView,
        config::{ProtocolConfigView, RpcProtocolConfigRequest},
        contract::ContractCodeView,
        network::StatusResponse,
        query::{FunctionArgs, QueryRequest, QueryResponseKind, RpcQueryRequest},
        state_change::{
            RpcStateChangesInBlockByTypeRequest,
            StateChangesRequestView,
            StateChangesView,
        },
        transaction::{CallResult, FinalExecutionOutcomeView, SignedTransaction, ViewStateResult},
        types::{AccountId, BlockReference, StoreKey},
    },
    json_rpc_client::{RpcError, Transport},
    near_anywhere_crypto::PublicKey,
    near_primitives_core::hash::CryptoHash,
};

#[derive(Clone)]
pub struct NearRpcUser {
    client: NearRpcClient,
}

impl NearRpcUser {
    pub fn new_with_http(url: &str) -> Self {
        let client = NearRpcClient::new_with_http(url);
        Self { client }
    }
    pub fn new_with_transport(transport: Transport) -> Self {
        let client = NearRpcClient::new(transport);
        Self { client }
    }

    pub async fn get_node_status(&self) -> Result<StatusResponse, RpcError> {
        self.client.status().await
    }

    pub async fn get_block(&self, block_reference: BlockReference) -> Result<BlockView, RpcError> {
        self.client.block(block_reference).await
    }

    pub async fn get_protocol_config(
        &self,
        block_reference: BlockReference,
    ) -> Result<ProtocolConfigView, RpcError> {
        let request = RpcProtocolConfigRequest { block_reference };
        let response = self.client.EXPERIMENTAL_protocol_config(request).await;

        Ok(response.config_view)
    }
}

// Queries
impl NearRpcUser {
    pub async fn view_access_key(
        &self,
        account_id: &AccountId,
        public_key: &PublicKey,
    ) -> Result<AccessKeyView, String> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewAccessKey {
                account_id: account_id.clone(),
                public_key: public_key.clone(),
            },
        };
        let query_response = self.client.query(request).await;

        match query_response.kind {
            QueryResponseKind::AccessKey(access_key) => Ok(access_key),
            _ => Err("Invalid type of response".into()),
        }
    }

    pub async fn view_access_key_list(
        &self,
        account_id: &AccountId,
    ) -> Result<AccessKeyList, RpcError> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewAccessKeyList {
                account_id: account_id.clone(),
            },
        };
        let query_response = self.client.query(request).await;

        match query_response.kind {
            QueryResponseKind::AccessKeyList(access_key_list) => Ok(access_key_list),
            _ => Err(RpcError {
                code: 0,
                message: "Invalid type of response".into(),
            }),
        }
    }

    pub async fn view_account(&self, account_id: &AccountId) -> Result<AccountView, RpcError> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewAccount {
                account_id: account_id.clone(),
            },
        };
        let query_response = self.client.query(request).await;

        match query_response.kind {
            QueryResponseKind::ViewAccount(account_view) => Ok(account_view),
            _ => Err(RpcError {
                code: 0,
                message: "Invalid type of response".into(),
            }),
        }
    }

    pub async fn view_contract_code(
        &self,
        account_id: &AccountId,
    ) -> Result<ContractCodeView, RpcError> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewCode {
                account_id: account_id.clone(),
            },
        };
        let query_response = self.client.query(request).await;

        match query_response.kind {
            QueryResponseKind::ViewCode(contract_code_view) => Ok(contract_code_view),
            _ => Err(RpcError {
                code: 0,
                message: "Invalid type of response".into(),
            }),
        }
    }

    pub async fn view_state(
        &self,
        account_id: &AccountId,
        prefix: StoreKey,
    ) -> Result<ViewStateResult, RpcError> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::ViewState {
                account_id: account_id.clone(),
                prefix: prefix.clone(),
            },
        };
        let query_response = self.client.query(request).await;

        match query_response.kind {
            QueryResponseKind::ViewState(contract_state_view) => Ok(contract_state_view),
            _ => Err(RpcError {
                code: 0,
                message: "Invalid type of response".into(),
            }),
        }
    }

    pub async fn view_call(
        &self,
        account_id: &AccountId,
        method_name: &str,
        args: FunctionArgs,
    ) -> Result<CallResult, RpcError> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: QueryRequest::CallFunction {
                account_id: account_id.clone(),
                method_name: method_name.to_string(),
                args,
            },
        };
        let query_response = self.client.query(request).await;

        match query_response.kind {
            QueryResponseKind::CallResult(call_result) => Ok(call_result),
            _ => Err(RpcError {
                code: 0,
                message: "Invalid type of response".into(),
            }),
        }
    }
}

// Changes
impl NearRpcUser {
    pub async fn view_single_access_key_changes(
        &self,
        keys: Vec<AccountWithPublicKey>,
    ) -> Result<StateChangesView, RpcError> {
        let request = StateChangesRequestView::SingleAccessKeyChanges { keys };
        let changes_request = RpcStateChangesInBlockByTypeRequest {
            block_reference: BlockReference::latest(),
            state_changes_request: request,
        };
        let changes_responses = self.client.EXPERIMENTAL_changes(changes_request).await;

        Ok(changes_responses.changes)
    }

    pub async fn view_access_key_changes(
        &self,
        account_ids: Vec<AccountId>,
    ) -> Result<StateChangesView, RpcError> {
        let request = StateChangesRequestView::AllAccessKeyChanges { account_ids };
        let changes_request = RpcStateChangesInBlockByTypeRequest {
            block_reference: BlockReference::latest(),
            state_changes_request: request,
        };
        let changes_responses = self.client.EXPERIMENTAL_changes(changes_request).await;

        Ok(changes_responses.changes)
    }

    pub async fn view_account_changes(
        &self,
        account_ids: Vec<AccountId>,
    ) -> Result<StateChangesView, RpcError> {
        let request = StateChangesRequestView::AccountChanges { account_ids };
        let changes_request = RpcStateChangesInBlockByTypeRequest {
            block_reference: BlockReference::latest(),
            state_changes_request: request,
        };
        let changes_responses = self.client.EXPERIMENTAL_changes(changes_request).await;

        Ok(changes_responses.changes)
    }

    pub async fn view_contract_code_changes(
        &self,
        account_ids: Vec<AccountId>,
    ) -> Result<StateChangesView, RpcError> {
        let request = StateChangesRequestView::ContractCodeChanges { account_ids };
        let changes_request = RpcStateChangesInBlockByTypeRequest {
            block_reference: BlockReference::latest(),
            state_changes_request: request,
        };
        let changes_responses = self.client.EXPERIMENTAL_changes(changes_request).await;

        Ok(changes_responses.changes)
    }

    pub async fn view_contract_state_changes(
        &self,
        account_ids: Vec<AccountId>,
        key_prefix: StoreKey,
    ) -> Result<StateChangesView, RpcError> {
        let request = StateChangesRequestView::DataChanges {
            key_prefix,
            account_ids,
        };
        let changes_request = RpcStateChangesInBlockByTypeRequest {
            block_reference: BlockReference::latest(),
            state_changes_request: request,
        };
        let changes_responses = self.client.EXPERIMENTAL_changes(changes_request).await;

        Ok(changes_responses.changes)
    }
}

// Transactions
impl NearRpcUser {
    pub async fn send_transaction_async(
        &self,
        tx: SignedTransaction,
    ) -> Result<CryptoHash, RpcError> {
        self.client.broadcast_tx_async(tx).await
    }

    pub async fn send_transaction(
        &self,
        tx: SignedTransaction,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        self.client.broadcast_tx_commit(tx).await
    }

    pub async fn get_transaction_status(
        &self,
        hash: CryptoHash,
        account_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        self.client.tx(hash, account_id).await
    }

    pub async fn get_transaction_status_with_receipts(
        &self,
        hash: CryptoHash,
        account_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        self.client.EXPERIMENTAL_tx_status(hash, account_id).await
    }
}

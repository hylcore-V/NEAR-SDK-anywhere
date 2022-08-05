use {
    super::{full_access_key, funcation_call_access_key, AccountAuthorizedApp, AccountBalance},
    crate::{
        client::RpcError,
        crypto::PublicKey,
        near::Connection,
        primitives::{
            access_key::{
                AccessKeyInfoView,
                AccessKeyList,
                AccessKeyPermissionView,
                AccessKeyView,
            },
            account::AccountView,
            actions::{
                Action,
                AddKeyAction,
                CreateAccountAction,
                DeleteAccountAction,
                DeleteKeyAction,
                DeployContractAction,
                FunctionCallAction,
                StakeAction,
                TransferAction,
            },
            near::NearRpcUser,
            query::FunctionArgs,
            transaction::{
                CallResult,
                FinalExecutionOutcomeView,
                SignedTransaction,
                ViewStateResult,
            },
            types::{AccountId, Balance, BlockReference, Finality, Gas, StoreKey},
        },
        transaction::sign_transaction_with_receiver,
    },
    hashbrown::HashMap,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Account {
    near_rpc_user: NearRpcUser,
    connection: Connection,
    account_id: AccountId,
    access_key_by_public_key_cache: HashMap<String, AccessKeyView>,
}

impl Account {
    pub fn new(connection: Connection, account_id: AccountId) -> Self {
        let near_rpc_user = NearRpcUser::new_with_transport(connection.transport.clone());

        Self {
            near_rpc_user,
            connection,
            account_id,
            access_key_by_public_key_cache: HashMap::new(),
        }
    }

    pub async fn state(&self) -> Result<AccountView, RpcError> {
        self.near_rpc_user.view_account(&self.account_id).await
    }

    pub async fn sign_transaction(
        &mut self,
        receiver_id: &str,
        actions: Vec<Action>,
    ) -> Result<SignedTransaction, RpcError> {
        let signer = self.connection.signer.clone();
        let network_id = self.connection.network_id.clone();
        let account_id = self.account_id.clone();
        let access_key_info = self
            .find_function_access_key(receiver_id, actions.clone())
            .await?;
        let access_key = access_key_info.access_key;
        let block = self
            .near_rpc_user
            .get_block(BlockReference::Finality(Finality::Final))
            .await?;
        let block_hash = block.header.hash;
        let nonce = access_key.nonce + 1;
        let signed_transaction = sign_transaction_with_receiver(
            receiver_id,
            nonce,
            actions,
            block_hash,
            signer,
            &account_id,
            &network_id,
        )
        .await;

        Ok(signed_transaction)
    }

    pub async fn sign_and_send_transaction(
        &mut self,
        receiver_id: &str,
        actions: Vec<Action>,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let signed_transaction = self.sign_transaction(&receiver_id, actions).await.unwrap();

        self.near_rpc_user
            .send_transaction(signed_transaction)
            .await
    }

    pub async fn find_access_key(&mut self) -> Result<AccessKeyInfoView, RpcError> {
        let account_id = self.account_id.clone();
        let public_key = self
            .connection
            .signer
            .get_public_key(&account_id, &self.connection.network_id)
            .expect("missing public key");
        let key = public_key.to_string();
        let cached_access_key = self.access_key_by_public_key_cache.get(&key);

        match cached_access_key {
            Some(access_key) => Ok(AccessKeyInfoView {
                public_key,
                access_key: access_key.clone().into(),
            }),
            None => {
                let access_key_view = self
                    .near_rpc_user
                    .view_access_key(&account_id, &public_key)
                    .await
                    .unwrap();

                self.access_key_by_public_key_cache
                    .insert(public_key.to_string(), access_key_view.clone());

                Ok(AccessKeyInfoView {
                    public_key,
                    access_key: access_key_view,
                })
            }
        }
    }

    pub async fn find_function_access_key(
        &mut self,
        receiver_id: &str,
        actions: Vec<Action>,
    ) -> Result<AccessKeyInfoView, RpcError> {
        self.find_access_key().await
    }

    pub async fn create_and_deploy_contract(
        &mut self,
        contract_id: String,
        public_key: PublicKey,
        data: &[u8],
        amount: Balance,
    ) -> Result<Account, RpcError> {
        let access_key = full_access_key();
        let actions = vec![
            Action::CreateAccount(CreateAccountAction {}),
            Action::Transfer(TransferAction { deposit: amount }),
            Action::AddKey(AddKeyAction {
                public_key,
                access_key,
            }),
            Action::DeployContract(DeployContractAction {
                code: data.to_vec(),
            }),
        ];
        let account_id = contract_id.clone();
        let contract_account_id = contract_id.clone().parse().unwrap();

        self.sign_and_send_transaction(&account_id, actions).await?;

        Ok(Account::new(self.connection.clone(), contract_account_id))
    }

    pub async fn send_money(
        &mut self,
        receiver_id: &str,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let result = self
            .sign_and_send_transaction(
                receiver_id,
                vec![Action::Transfer(TransferAction { deposit: amount })],
            )
            .await?;

        Ok(result)
    }

    pub async fn create_account(
        &mut self,
        account_id: &str,
        public_key: PublicKey,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let access_key = full_access_key();
        let actions = vec![
            Action::CreateAccount(CreateAccountAction {}),
            Action::Transfer(TransferAction { deposit: amount }),
            Action::AddKey(AddKeyAction {
                public_key,
                access_key,
            }),
        ];
        let result = self.sign_and_send_transaction(account_id, actions).await?;

        Ok(result)
    }

    pub async fn delete_account(
        &mut self,
        beneficiary_id: &str,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let account_id = self.account_id.clone();
        let beneficiary_id = beneficiary_id.parse().unwrap();
        let result = self
            .sign_and_send_transaction(
                &account_id,
                vec![Action::DeleteAccount(DeleteAccountAction {
                    beneficiary_id,
                })],
            )
            .await?;

        Ok(result)
    }

    pub async fn deploy_contract(
        &mut self,
        data: &[u8],
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let account_id = self.account_id.clone();
        let result = self
            .sign_and_send_transaction(
                &account_id,
                vec![Action::DeployContract(DeployContractAction {
                    code: data.to_vec(),
                })],
            )
            .await?;

        Ok(result)
    }

    pub async fn function_call(
        &mut self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        gas: Gas,
        deposit: Balance,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let result = self
            .sign_and_send_transaction(
                contract_id,
                vec![Action::FunctionCall(FunctionCallAction {
                    method_name: method_name.to_string(),
                    args,
                    gas,
                    deposit,
                })],
            )
            .await?;

        Ok(result)
    }

    pub async fn add_key(
        &mut self,
        public_key: PublicKey,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let account_id = self.account_id.clone();
        let access_key = full_access_key();
        let result = self
            .sign_and_send_transaction(
                &account_id,
                vec![Action::AddKey(AddKeyAction {
                    public_key,
                    access_key,
                })],
            )
            .await?;

        Ok(result)
    }

    pub async fn add_key_to_contract(
        &mut self,
        public_key: PublicKey,
        contract_id: &str,
        method_names: Vec<String>,
        amount: Option<Balance>,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let access_key = funcation_call_access_key(contract_id.to_string(), method_names, amount);
        let result = self
            .sign_and_send_transaction(
                contract_id,
                vec![Action::AddKey(AddKeyAction {
                    public_key,
                    access_key,
                })],
            )
            .await?;

        Ok(result)
    }

    pub async fn delete_key(
        &mut self,
        public_key: PublicKey,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let account_id = self.account_id.clone();
        let result = self
            .sign_and_send_transaction(
                &account_id,
                vec![Action::DeleteKey(DeleteKeyAction { public_key })],
            )
            .await?;

        Ok(result)
    }

    pub async fn stake(
        &mut self,
        public_key: PublicKey,
        amount: Balance,
    ) -> Result<FinalExecutionOutcomeView, RpcError> {
        let account_id = self.account_id.clone();
        let result = self
            .sign_and_send_transaction(
                &account_id,
                vec![Action::Stake(StakeAction {
                    public_key,
                    stake: amount,
                })],
            )
            .await?;

        Ok(result)
    }

    pub async fn view_function(
        &self,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<CallResult, RpcError> {
        let account_id = contract_id.parse().unwrap();
        let view_call_result = self
            .near_rpc_user
            .view_call(&account_id, method_name, FunctionArgs(args))
            .await?;

        Ok(view_call_result)
    }

    pub async fn view_state(&self, prefix: StoreKey) -> Result<ViewStateResult, RpcError> {
        let account_id = self.account_id.clone();
        let view_state_result = self.near_rpc_user.view_state(&account_id, prefix).await?;

        Ok(view_state_result)
    }

    pub async fn get_access_keys(&self) -> Result<AccessKeyList, RpcError> {
        let account_id = self.account_id.clone();
        let access_key_list = self.near_rpc_user.view_access_key_list(&account_id).await?;

        Ok(access_key_list)
    }

    pub async fn get_account_details(&self) -> Result<Vec<AccountAuthorizedApp>, RpcError> {
        let access_keys = self.get_access_keys().await?;
        let authorized_apps = access_keys
            .keys
            .iter()
            .filter(|item| item.access_key.permission != AccessKeyPermissionView::FullAccess)
            .map(|item| match item.access_key.permission.clone() {
                AccessKeyPermissionView::FunctionCall {
                    allowance,
                    receiver_id,
                    method_names: _,
                } => AccountAuthorizedApp {
                    contract_id: receiver_id.parse().unwrap(),
                    amount: allowance.unwrap_or(0),
                    public_key: item.public_key.clone(),
                },
                AccessKeyPermissionView::FullAccess => todo!(),
            })
            .collect::<Vec<AccountAuthorizedApp>>();

        Ok(authorized_apps)
    }

    pub async fn get_account_balance(&self) -> Result<AccountBalance, RpcError> {
        let protocol_config = self
            .near_rpc_user
            .get_protocol_config(BlockReference::Finality(Finality::Final))
            .await?;
        let state = self.state().await?;

        let cost_per_byte = protocol_config.runtime_config.storage_amount_per_byte;
        let state_staked = state.storage_usage as u128 * cost_per_byte;
        let staked = state.locked;
        let total = state.amount + staked;
        let available = match staked > state_staked {
            true => total / staked,
            false => total / state_staked,
        };

        Ok(AccountBalance {
            total,
            state_staked,
            staked,
            available,
        })
    }
}

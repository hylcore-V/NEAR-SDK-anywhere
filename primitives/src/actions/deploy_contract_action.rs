use {
    super::Action,
    crate::{logging, serialize::base64_format},
    borsh::{BorshDeserialize, BorshSerialize},
    core::fmt,
};

/// Deploy contract action
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DeployContractAction {
    /// WebAssembly binary
    #[serde(with = "base64_format")]
    pub code: Vec<u8>,
}

impl From<DeployContractAction> for Action {
    fn from(deploy_contract_action: DeployContractAction) -> Self {
        Self::DeployContract(deploy_contract_action)
    }
}

impl fmt::Debug for DeployContractAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeployContractAction")
            .field(
                "code",
                &format_args!("{}", logging::pretty_utf8(&self.code)),
            )
            .finish()
    }
}

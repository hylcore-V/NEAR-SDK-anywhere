use {
    super::StatusSyncInfo,
    crate::{network::ValidatorInfo, version::Version},
    near_primitives_core::types::AccountId,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    /// Binary version.
    pub version: Version,
    /// Unique chain id.
    pub chain_id: String,
    /// Currently active protocol version.
    pub protocol_version: u32,
    /// Latest protocol version that this client supports.
    pub latest_protocol_version: u32,
    /// Address for RPC server.  None if node doesn’t have RPC endpoint enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_addr: Option<String>,
    /// Current epoch validators.
    pub validators: Vec<ValidatorInfo>,
    /// Sync status of the node.
    pub sync_info: StatusSyncInfo,
    /// Validator id of the node
    pub validator_account_id: Option<AccountId>,
    /// Information about last blocks, network, epoch and chain & chunk info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_debug_status: Option<String>,
}

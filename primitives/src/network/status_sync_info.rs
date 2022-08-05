use crate::{
    hash::CryptoHash,
    types::{BlockHeight, DateTime, EpochId},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusSyncInfo {
    pub latest_block_hash: CryptoHash,
    pub latest_block_height: BlockHeight,
    pub latest_state_root: CryptoHash,
    pub latest_block_time: DateTime<chrono::Utc>,
    pub syncing: bool,
    pub earliest_block_hash: Option<CryptoHash>,
    pub earliest_block_height: Option<BlockHeight>,
    pub earliest_block_time: Option<DateTime<chrono::Utc>>,
    pub epoch_id: Option<EpochId>,
    pub epoch_start_height: Option<BlockHeight>,
}

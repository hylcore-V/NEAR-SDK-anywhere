use {
    crate::types::{AccountId, BlockHeight, ShardId},
    near_anywhere_crypto::PublicKey,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PeerInfoView {
    pub addr: String,
    pub account_id: Option<AccountId>,
    pub height: BlockHeight,
    pub tracked_shards: Vec<ShardId>,
    pub archival: bool,
    pub peer_id: PublicKey,
}

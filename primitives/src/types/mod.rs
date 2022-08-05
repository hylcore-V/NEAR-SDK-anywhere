use crate::hash::CryptoHash;

mod block_id;
mod block_reference;
mod block_shard_id;
mod cost_gas_used;
mod epoch_id;
mod finality;
mod gas_price_view;
mod store_key;
mod store_value;

pub use {
    block_id::BlockId,
    block_reference::BlockReference,
    block_shard_id::BlockShardId,
    chrono::DateTime,
    cost_gas_used::CostGasUsed,
    epoch_id::EpochId,
    finality::Finality,
    gas_price_view::GasPriceView,
    near_primitives_core::types::*,
    store_key::StoreKey,
    store_value::StoreValue,
};

pub type BlockHash = CryptoHash;
pub type LogEntry = String;
pub type StateRoot = CryptoHash;
pub type TrieProofPath = Vec<String>;
pub type MaybeBlockId = Option<BlockId>;

mod direction;
mod merkle_path_item;
mod partial_merkle_tree;
mod tests;
mod utils;

pub use {
    direction::Direction,
    merkle_path_item::MerklePathItem,
    partial_merkle_tree::PartialMerkleTree,
    utils::{
        combine_hash,
        compute_root_from_path,
        compute_root_from_path_and_item,
        merklize,
        verify_hash,
        verify_path,
    },
};

pub type MerklePath = Vec<MerklePathItem>;

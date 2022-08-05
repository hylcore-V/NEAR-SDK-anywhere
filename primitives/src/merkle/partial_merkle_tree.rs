use {
    super::combine_hash,
    crate::{hash::CryptoHash, types::MerkleHash},
    borsh::{BorshDeserialize, BorshSerialize},
};

/// Merkle tree that only maintains the path for the next leaf, i.e,
/// when a new leaf is inserted, the existing `path` is its proof.
/// The root can be computed by folding `path` from right but is not explicitly
/// maintained to save space.
/// The size of the object is O(log(n)) where n is the number of leaves in the tree, i.e, `size`.
#[cfg_attr(feature = "deepsize_feature", derive(deepsize::DeepSizeOf))]
#[derive(Default, Clone, BorshSerialize, BorshDeserialize, Eq, PartialEq, Debug)]
pub struct PartialMerkleTree {
    /// Path for the next leaf.
    path: Vec<MerkleHash>,
    /// Number of leaves in the tree.
    size: u64,
}

impl PartialMerkleTree {
    pub fn root(&self) -> MerkleHash {
        if self.path.is_empty() {
            CryptoHash::default()
        } else {
            let mut res = *self.path.last().unwrap();
            let len = self.path.len();
            for i in (0..len - 1).rev() {
                res = combine_hash(&self.path[i], &res);
            }
            res
        }
    }

    pub fn insert(&mut self, elem: MerkleHash) {
        let mut s = self.size;
        let mut node = elem;
        while s % 2 == 1 {
            let last_path_elem = self.path.pop().unwrap();
            node = combine_hash(&last_path_elem, &node);
            s /= 2;
        }
        self.path.push(node);
        self.size += 1;
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn get_path(&self) -> &[MerkleHash] {
        &self.path
    }
}

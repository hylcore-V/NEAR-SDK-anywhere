#[cfg(test)]
mod tests {
    use {
        crate::{
            hash::{hash, CryptoHash},
            merkle::*,
            types::MerkleHash,
        },
        rand::{rngs::StdRng, Rng, SeedableRng},
    };

    fn test_with_len(n: u32, rng: &mut StdRng) {
        let mut arr: Vec<u32> = vec![];
        for _ in 0..n {
            arr.push(rng.gen_range(0..1000));
        }
        let (root, paths) = merklize(&arr);
        assert_eq!(paths.len() as u32, n);
        for (i, item) in arr.iter().enumerate() {
            assert!(verify_path(root, &paths[i], item));
        }
    }

    #[test]
    fn test_merkle_path() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(1);
        for _ in 0..10 {
            let len: u32 = rng.gen_range(1..100);
            test_with_len(len, &mut rng);
        }
    }

    #[test]
    fn test_incorrect_path() {
        let items = vec![111, 222, 333];
        let (root, paths) = merklize(&items);
        for i in 0..items.len() {
            assert!(!verify_path(root, &paths[(i + 1) % 3], &items[i]))
        }
    }

    #[test]
    fn test_elements_order() {
        let items = vec![1, 2];
        let (root, _) = merklize(&items);
        let items2 = vec![2, 1];
        let (root2, _) = merklize(&items2);
        assert_ne!(root, root2);
    }

    /// Compute the merkle root of a given array.
    fn compute_root(hashes: &[CryptoHash]) -> CryptoHash {
        if hashes.is_empty() {
            CryptoHash::default()
        } else if hashes.len() == 1 {
            hashes[0]
        } else {
            let len = hashes.len();
            let subtree_len = len.next_power_of_two() / 2;
            let left_root = compute_root(&hashes[0..subtree_len]);
            let right_root = compute_root(&hashes[subtree_len..len]);
            combine_hash(&left_root, &right_root)
        }
    }

    #[test]
    fn test_merkle_tree() {
        let mut tree = PartialMerkleTree::default();
        let mut hashes = vec![];
        for i in 0..50 {
            assert_eq!(compute_root(&hashes), tree.root());
            let cur_hash = hash(&[i]);
            hashes.push(cur_hash);
            tree.insert(cur_hash);
        }
    }

    #[test]
    fn test_combine_hash_stability() {
        let a = MerkleHash::default();
        let b = MerkleHash::default();
        let cc = combine_hash(&a, &b);
        assert_eq!(
            cc.0,
            [
                245, 165, 253, 66, 209, 106, 32, 48, 39, 152, 239, 110, 211, 9, 151, 155, 67, 0,
                61, 35, 32, 217, 240, 232, 234, 152, 49, 169, 39, 89, 251, 75
            ]
        );
    }
}

use {
    super::{Direction, MerklePath, MerklePathItem},
    crate::{
        hash::{hash, CryptoHash},
        types::MerkleHash,
    },
    borsh::BorshSerialize,
};

pub fn combine_hash(hash1: &MerkleHash, hash2: &MerkleHash) -> MerkleHash {
    CryptoHash::hash_borsh(&(hash1, hash2))
}

/// Merklize an array of items. If the array is empty, returns hash of 0
pub fn merklize<T: BorshSerialize>(arr: &[T]) -> (MerkleHash, Vec<MerklePath>) {
    if arr.is_empty() {
        return (MerkleHash::default(), vec![]);
    }
    let mut len = arr.len().next_power_of_two();
    let mut hashes = arr.iter().map(CryptoHash::hash_borsh).collect::<Vec<_>>();

    // degenerate case
    if len == 1 {
        return (hashes[0], vec![vec![]]);
    }
    let mut arr_len = arr.len();
    let mut paths: Vec<MerklePath> = (0..arr_len)
        .map(|i| {
            if i % 2 == 0 {
                if i + 1 < arr_len {
                    vec![MerklePathItem {
                        hash: hashes[(i + 1) as usize],
                        direction: Direction::Right,
                    }]
                } else {
                    vec![]
                }
            } else {
                vec![MerklePathItem {
                    hash: hashes[(i - 1) as usize],
                    direction: Direction::Left,
                }]
            }
        })
        .collect();

    let mut counter = 1;
    while len > 1 {
        len /= 2;
        counter *= 2;
        for i in 0..len {
            let hash = if 2 * i >= arr_len {
                continue;
            } else if 2 * i + 1 >= arr_len {
                hashes[2 * i]
            } else {
                combine_hash(&hashes[2 * i], &hashes[2 * i + 1])
            };
            hashes[i] = hash;
            if len > 1 {
                if i % 2 == 0 {
                    for j in 0..counter {
                        let index = ((i + 1) * counter + j) as usize;
                        if index < arr.len() {
                            paths[index].push(MerklePathItem {
                                hash,
                                direction: Direction::Left,
                            });
                        }
                    }
                } else {
                    for j in 0..counter {
                        let index = ((i - 1) * counter + j) as usize;
                        if index < arr.len() {
                            paths[index].push(MerklePathItem {
                                hash,
                                direction: Direction::Right,
                            });
                        }
                    }
                }
            }
        }
        arr_len = (arr_len + 1) / 2;
    }
    (hashes[0], paths)
}

/// Verify merkle path for given item and corresponding path.
pub fn verify_path<T: BorshSerialize>(root: MerkleHash, path: &MerklePath, item: &T) -> bool {
    let hash = hash(&item.try_to_vec().expect("Failed to serialize"));
    verify_hash(root, path, hash)
}

pub fn verify_hash(root: MerkleHash, path: &MerklePath, item_hash: MerkleHash) -> bool {
    compute_root_from_path(path, item_hash) == root
}

pub fn compute_root_from_path(path: &MerklePath, item_hash: MerkleHash) -> MerkleHash {
    let mut res = item_hash;
    for item in path {
        match item.direction {
            Direction::Left => {
                res = combine_hash(&item.hash, &res);
            }
            Direction::Right => {
                res = combine_hash(&res, &item.hash);
            }
        }
    }
    res
}

pub fn compute_root_from_path_and_item<T: BorshSerialize>(
    path: &MerklePath,
    item: &T,
) -> MerkleHash {
    let hash = hash(&item.try_to_vec().expect("Failed to serialize"));
    compute_root_from_path(path, hash)
}

use {
    crate::merkle::{merklize, MerklePath},
    borsh::{BorshDeserialize, BorshSerialize},
    near_primitives_core::types::MerkleHash,
};

#[derive(Default, BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EncodedShardChunkBody {
    pub parts: Vec<Option<Box<[u8]>>>,
}

impl EncodedShardChunkBody {
    pub fn num_fetched_parts(&self) -> usize {
        let mut fetched_parts: usize = 0;

        for part in self.parts.iter() {
            if part.is_some() {
                fetched_parts += 1;
            }
        }

        fetched_parts
    }

    /// Returns true if reconstruction was successful
    /// TODO: we dont need it
    // pub fn reconstruct(
    //     &mut self,
    //     rs: &mut ReedSolomonWrapper,
    // ) -> Result<(), reed_solomon_erasure::Error> {
    //     rs.reconstruct(self.parts.as_mut_slice())
    // }

    pub fn get_merkle_hash_and_paths(&self) -> (MerkleHash, Vec<MerklePath>) {
        let parts: Vec<&[u8]> = self
            .parts
            .iter()
            .map(|x| x.as_deref().unwrap())
            .collect::<Vec<_>>();
        merklize(&parts)
    }
}

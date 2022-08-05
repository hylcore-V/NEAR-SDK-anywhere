use {
    crate::hash::CryptoHash,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Debug,
    Default,
)]
pub struct ChunkHash(pub CryptoHash);

impl ChunkHash {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0 .0
    }
}

impl AsRef<[u8]> for ChunkHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<ChunkHash> for Vec<u8> {
    fn from(chunk_hash: ChunkHash) -> Self {
        chunk_hash.0.into()
    }
}

impl From<CryptoHash> for ChunkHash {
    fn from(crypto_hash: CryptoHash) -> Self {
        Self(crypto_hash)
    }
}

use {
    super::SECP256K1_PUBLIC_KEY_LENGTH,
    borsh::{BorshDeserialize, BorshSerialize},
    core::{cmp::Ordering, convert::TryFrom},
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Hash)]
pub struct Secp256K1PublicKey(pub [u8; SECP256K1_PUBLIC_KEY_LENGTH]);

impl From<&[u8]> for Secp256K1PublicKey {
    fn from(data: &[u8]) -> Self {
        Self(<[u8; SECP256K1_PUBLIC_KEY_LENGTH]>::try_from(<&[u8]>::clone(&data)).unwrap())
    }
}

impl AsRef<[u8]> for Secp256K1PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Debug for Secp256K1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", bs58::encode(&self.0.to_vec()).into_string())
    }
}

impl From<Secp256K1PublicKey> for [u8; 64] {
    fn from(pubkey: Secp256K1PublicKey) -> Self {
        pubkey.0
    }
}

impl Eq for Secp256K1PublicKey {}

impl PartialEq for Secp256K1PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0[..].eq(&other.0[..])
    }
}

impl PartialOrd for Secp256K1PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0[..].partial_cmp(&other.0[..])
    }
}

impl Ord for Secp256K1PublicKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0[..].cmp(&other.0[..])
    }
}

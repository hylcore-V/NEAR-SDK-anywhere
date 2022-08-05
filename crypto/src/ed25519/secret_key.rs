use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Eq, Ord, PartialOrd, Hash)]
pub struct ED25519SecretKey(pub [u8; ed25519_dalek::KEYPAIR_LENGTH]);

impl PartialEq for ED25519SecretKey {
    fn eq(&self, other: &Self) -> bool {
        self.0[..ed25519_dalek::SECRET_KEY_LENGTH] == other.0[..ed25519_dalek::SECRET_KEY_LENGTH]
    }
}

impl core::fmt::Debug for ED25519SecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            bs58::encode(&self.0[..ed25519_dalek::SECRET_KEY_LENGTH].to_vec()).into_string()
        )
    }
}

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(
    BorshSerialize,
    BorshDeserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    derive_more::AsRef,
)]
#[as_ref(forward)]
pub struct ED25519PublicKey(pub [u8; ed25519_dalek::PUBLIC_KEY_LENGTH]);

impl From<[u8; ed25519_dalek::PUBLIC_KEY_LENGTH]> for ED25519PublicKey {
    fn from(data: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH]) -> Self {
        Self(data)
    }
}

impl TryFrom<&[u8]> for ED25519PublicKey {
    type Error = crate::ParseKeyError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(data.try_into().map_err(|_| {
            Self::Error::InvalidLength {
                expected_length: ed25519_dalek::PUBLIC_KEY_LENGTH,
                received_length: data.len(),
            }
        })?))
    }
}

impl core::fmt::Debug for ED25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", bs58::encode(&self.0.to_vec()).into_string())
    }
}

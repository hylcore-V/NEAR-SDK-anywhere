use {
    crate::{
        ed25519::ED25519PublicKey,
        key_type::{split_key_type_data, KeyType},
        secp256k1::Secp256K1PublicKey,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    core::{
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        str::FromStr,
    },
    std::io::{Error, ErrorKind, Write},
};

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum PublicKey {
    /// 256 bit elliptic curve based public-key.
    ED25519(ED25519PublicKey),
    /// 512 bit elliptic curve based public-key used in Bitcoin's public-key cryptography.
    SECP256K1(Secp256K1PublicKey),
}

impl PublicKey {
    pub fn len(&self) -> usize {
        match self {
            Self::ED25519(_) => ed25519_dalek::PUBLIC_KEY_LENGTH + 1,
            Self::SECP256K1(_) => 65,
        }
    }

    pub fn empty(key_type: KeyType) -> Self {
        match key_type {
            KeyType::ED25519 => {
                PublicKey::ED25519(ED25519PublicKey([0u8; ed25519_dalek::PUBLIC_KEY_LENGTH]))
            }
            KeyType::SECP256K1 => PublicKey::SECP256K1(Secp256K1PublicKey([0u8; 64])),
        }
    }

    pub fn key_type(&self) -> KeyType {
        match self {
            Self::ED25519(_) => KeyType::ED25519,
            Self::SECP256K1(_) => KeyType::SECP256K1,
        }
    }

    pub fn key_data(&self) -> &[u8] {
        match self {
            Self::ED25519(key) => key.as_ref(),
            Self::SECP256K1(key) => key.as_ref(),
        }
    }

    pub fn unwrap_as_ed25519(&self) -> &ED25519PublicKey {
        match self {
            Self::ED25519(key) => key,
            Self::SECP256K1(_) => panic!(),
        }
    }
}

// This `Hash` implementation is safe since it retains the property
// `k1 == k2 ⇒ hash(k1) == hash(k2)`.
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PublicKey::ED25519(public_key) => {
                state.write_u8(0u8);
                state.write(&public_key.0);
            }
            PublicKey::SECP256K1(public_key) => {
                state.write_u8(1u8);
                state.write(&public_key.0);
            }
        }
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", String::from(self))
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", String::from(self))
    }
}

impl BorshSerialize for PublicKey {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            PublicKey::ED25519(public_key) => {
                BorshSerialize::serialize(&0u8, writer)?;
                writer.write_all(&public_key.0)?;
            }
            PublicKey::SECP256K1(public_key) => {
                BorshSerialize::serialize(&1u8, writer)?;
                writer.write_all(&public_key.0)?;
            }
        }
        Ok(())
    }
}

impl BorshDeserialize for PublicKey {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, Error> {
        let key_type = KeyType::try_from(<u8 as BorshDeserialize>::deserialize(buf)?)
            .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;
        match key_type {
            KeyType::ED25519 => Ok(PublicKey::ED25519(ED25519PublicKey(
                BorshDeserialize::deserialize(buf)?,
            ))),
            KeyType::SECP256K1 => Ok(PublicKey::SECP256K1(Secp256K1PublicKey(
                BorshDeserialize::deserialize(buf)?,
            ))),
        }
    }
}

impl serde::Serialize for PublicKey {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl<'de> serde::Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        s.parse()
            .map_err(|err: crate::errors::ParseKeyError| serde::de::Error::custom(err.to_string()))
    }
}

impl From<&PublicKey> for String {
    fn from(public_key: &PublicKey) -> Self {
        match public_key {
            PublicKey::ED25519(public_key) => {
                format!(
                    "{}:{}",
                    KeyType::ED25519,
                    bs58::encode(&public_key.0).into_string()
                )
            }
            PublicKey::SECP256K1(public_key) => format!(
                "{}:{}",
                KeyType::SECP256K1,
                bs58::encode(&public_key.0.to_vec()).into_string()
            ),
        }
    }
}

impl FromStr for PublicKey {
    type Err = crate::ParseKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (key_type, key_data) = split_key_type_data(value)?;
        match key_type {
            KeyType::ED25519 => {
                let mut array = [0; ed25519_dalek::PUBLIC_KEY_LENGTH];
                let length = bs58::decode(key_data).into(&mut array).map_err(|err| {
                    Self::Err::InvalidData {
                        error_message: err.to_string(),
                    }
                })?;
                if length != ed25519_dalek::PUBLIC_KEY_LENGTH {
                    return Err(Self::Err::InvalidLength {
                        expected_length: ed25519_dalek::PUBLIC_KEY_LENGTH,
                        received_length: length,
                    });
                }
                Ok(PublicKey::ED25519(ED25519PublicKey(array)))
            }
            KeyType::SECP256K1 => {
                let mut array = [0; 64];
                let length = bs58::decode(key_data).into(&mut array[..]).map_err(|err| {
                    Self::Err::InvalidData {
                        error_message: err.to_string(),
                    }
                })?;
                if length != 64 {
                    return Err(Self::Err::InvalidLength {
                        expected_length: 64,
                        received_length: length,
                    });
                }
                Ok(PublicKey::SECP256K1(Secp256K1PublicKey(array)))
            }
        }
    }
}

impl From<ED25519PublicKey> for PublicKey {
    fn from(ed25519: ED25519PublicKey) -> Self {
        Self::ED25519(ed25519)
    }
}

impl From<Secp256K1PublicKey> for PublicKey {
    fn from(secp256k1: Secp256K1PublicKey) -> Self {
        Self::SECP256K1(secp256k1)
    }
}

use {
    crate::{
        key_type::{split_key_type_data, KeyType},
        secp256k1::Secp256K1Signature,
        ParseSignatureError,
        PublicKey,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    core::{
        fmt::{Debug, Display, Formatter},
        hash::{Hash, Hasher},
        str::FromStr,
    },
    std::io::{Error, ErrorKind, Write},
};

/// Signature container supporting different curves.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Signature {
    ED25519(ed25519_dalek::Signature),
    SECP256K1(Secp256K1Signature),
}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Signature::ED25519(sig) => sig.to_bytes().hash(state),
            Signature::SECP256K1(sig) => sig.hash(state),
        };
    }
}

impl Signature {
    /// Construct Signature from key type and raw signature blob
    pub fn from_parts(
        signature_type: KeyType,
        signature_data: &[u8],
    ) -> Result<Self, ParseSignatureError> {
        match signature_type {
            KeyType::ED25519 => Ok(Signature::ED25519(
                ed25519_dalek::Signature::from_bytes(signature_data).map_err(|err| {
                    ParseSignatureError::InvalidData {
                        error_message: err.to_string(),
                    }
                })?,
            )),
            KeyType::SECP256K1 => Ok(Signature::SECP256K1(
                Secp256K1Signature::try_from(signature_data).map_err(|_| {
                    ParseSignatureError::InvalidData {
                        error_message: "invalid Secp256k1 signature length".to_string(),
                    }
                })?,
            )),
        }
    }

    /// Verifies that this signature is indeed signs the data with given public key.
    /// Also if public key doesn't match on the curve returns `false`.
    pub fn verify(&self, data: &[u8], public_key: &PublicKey) -> bool {
        match (&self, public_key) {
            (Signature::ED25519(signature), PublicKey::ED25519(public_key)) => {
                match ed25519_dalek::PublicKey::from_bytes(&public_key.0) {
                    Err(_) => false,
                    Ok(public_key) => public_key.verify_strict(data, signature).is_ok(),
                }
            }
            (Signature::SECP256K1(signature), PublicKey::SECP256K1(public_key)) => {
                signature.verify(data, &public_key.0).is_ok()
            }
            _ => false,
        }
    }

    pub fn key_type(&self) -> KeyType {
        match self {
            Signature::ED25519(_) => KeyType::ED25519,
            Signature::SECP256K1(_) => KeyType::SECP256K1,
        }
    }
}

impl FromStr for Signature {
    type Err = ParseSignatureError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (sig_type, sig_data) = split_key_type_data(value)?;
        match sig_type {
            KeyType::ED25519 => {
                let mut array = [0; ed25519_dalek::SIGNATURE_LENGTH];
                let length = bs58::decode(sig_data).into(&mut array[..]).map_err(|err| {
                    Self::Err::InvalidData {
                        error_message: err.to_string(),
                    }
                })?;
                if length != ed25519_dalek::SIGNATURE_LENGTH {
                    return Err(Self::Err::InvalidLength {
                        expected_length: ed25519_dalek::SIGNATURE_LENGTH,
                        received_length: length,
                    });
                }
                Ok(Signature::ED25519(
                    ed25519_dalek::Signature::from_bytes(&array).map_err(|err| {
                        Self::Err::InvalidData {
                            error_message: err.to_string(),
                        }
                    })?,
                ))
            }
            KeyType::SECP256K1 => {
                let mut array = [0; 65];
                let length = bs58::decode(sig_data).into(&mut array[..]).map_err(|err| {
                    Self::Err::InvalidData {
                        error_message: err.to_string(),
                    }
                })?;
                if length != 65 {
                    return Err(Self::Err::InvalidLength {
                        expected_length: 65,
                        received_length: length,
                    });
                }
                Ok(Signature::SECP256K1(Secp256K1Signature(array)))
            }
        }
    }
}

impl BorshSerialize for Signature {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Signature::ED25519(signature) => {
                BorshSerialize::serialize(&0u8, writer)?;
                writer.write_all(&signature.to_bytes())?;
            }
            Signature::SECP256K1(signature) => {
                BorshSerialize::serialize(&1u8, writer)?;
                writer.write_all(&signature.0)?;
            }
        }
        Ok(())
    }
}

impl BorshDeserialize for Signature {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, Error> {
        let key_type = KeyType::try_from(<u8 as BorshDeserialize>::deserialize(buf)?)
            .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;
        match key_type {
            KeyType::ED25519 => {
                let array: [u8; ed25519_dalek::SIGNATURE_LENGTH] =
                    BorshDeserialize::deserialize(buf)?;
                Ok(Signature::ED25519(
                    ed25519_dalek::Signature::from_bytes(&array)
                        .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?,
                ))
            }
            KeyType::SECP256K1 => {
                let array: [u8; 65] = BorshDeserialize::deserialize(buf)?;
                Ok(Signature::SECP256K1(Secp256K1Signature(array)))
            }
        }
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let data = match self {
            Signature::ED25519(signature) => {
                bs58::encode(&signature.to_bytes().to_vec()).into_string()
            }
            Signature::SECP256K1(signature) => bs58::encode(&signature.0[..]).into_string(),
        };
        write!(f, "{}", format!("{}:{}", self.key_type(), data))
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl serde::Serialize for Signature {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        s.parse()
            .map_err(|err: ParseSignatureError| serde::de::Error::custom(err.to_string()))
    }
}

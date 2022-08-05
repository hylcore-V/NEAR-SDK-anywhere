use {
    crate::{
        ed25519::{ED25519PublicKey, ED25519SecretKey},
        key_type::{split_key_type_data, KeyType},
        secp256k1::{Secp256K1PublicKey, Secp256K1Signature, SECP256K1_SECRET_KEY_SIZE},
        PublicKey,
        Signature,
    },
    core::{
        fmt::{Debug, Display},
        str::FromStr,
    },
    ed25519_dalek::Signer,
};

/// Secret key container supporting different curves.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SecretKey {
    ED25519(ED25519SecretKey),
    SECP256K1(libsecp256k1::SecretKey),
}

impl SecretKey {
    pub fn key_type(&self) -> KeyType {
        match self {
            SecretKey::ED25519(_) => KeyType::ED25519,
            SecretKey::SECP256K1(_) => KeyType::SECP256K1,
        }
    }

    pub fn from_random(key_type: KeyType) -> SecretKey {
        match key_type {
            KeyType::ED25519 => {
                let mut csprng = rand_07::rngs::OsRng {};
                let key_pair = ed25519_dalek::Keypair::generate(&mut csprng);
                SecretKey::ED25519(ED25519SecretKey(key_pair.to_bytes()))
            }
            KeyType::SECP256K1 => {
                SecretKey::SECP256K1(libsecp256k1::SecretKey::random(&mut rand::rngs::OsRng))
            }
        }
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        match &self {
            SecretKey::ED25519(secret_key) => {
                let keypair = ed25519_dalek::Keypair::from_bytes(&secret_key.0).unwrap();
                Signature::ED25519(keypair.sign(data))
            }

            SecretKey::SECP256K1(secret_key) => {
                let (_, recover_id) = libsecp256k1::sign(
                    &libsecp256k1::Message::parse_slice(data).expect("32 bytes"),
                    secret_key,
                );
                let mut buf = [0; 65];
                buf[0..64].copy_from_slice(&data[0..64]);
                buf[64] = recover_id.serialize();
                Signature::SECP256K1(Secp256K1Signature(buf))
            }
        }
    }

    pub fn public_key(&self) -> PublicKey {
        match &self {
            SecretKey::ED25519(secret_key) => PublicKey::ED25519(ED25519PublicKey(
                secret_key.0[ed25519_dalek::SECRET_KEY_LENGTH..]
                    .try_into()
                    .unwrap(),
            )),
            SecretKey::SECP256K1(secret_key) => {
                let pk = libsecp256k1::PublicKey::from_secret_key(secret_key);
                let serialized = pk.serialize();
                let mut public_key = Secp256K1PublicKey([0; 64]);
                public_key.0.copy_from_slice(&serialized[1..65]);
                PublicKey::SECP256K1(public_key)
            }
        }
    }

    pub fn unwrap_as_ed25519(&self) -> &ED25519SecretKey {
        match self {
            SecretKey::ED25519(key) => key,
            SecretKey::SECP256K1(_) => panic!(),
        }
    }
}

impl Display for SecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let data = match self {
            SecretKey::ED25519(secret_key) => bs58::encode(&secret_key.0[..]).into_string(),
            SecretKey::SECP256K1(secret_key) => {
                bs58::encode(&secret_key.serialize()[..]).into_string()
            }
        };
        write!(f, "{}:{}", self.key_type(), data)
    }
}

impl FromStr for SecretKey {
    type Err = crate::errors::ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key_type, key_data) = split_key_type_data(s)?;
        match key_type {
            KeyType::ED25519 => {
                let mut array = [0; ed25519_dalek::KEYPAIR_LENGTH];
                let length = bs58::decode(key_data).into(&mut array[..]).map_err(|err| {
                    Self::Err::InvalidData {
                        error_message: err.to_string(),
                    }
                })?;
                if length != ed25519_dalek::KEYPAIR_LENGTH {
                    return Err(Self::Err::InvalidLength {
                        expected_length: ed25519_dalek::KEYPAIR_LENGTH,
                        received_length: length,
                    });
                }
                Ok(Self::ED25519(ED25519SecretKey(array)))
            }
            KeyType::SECP256K1 => {
                let mut array = [0; SECP256K1_SECRET_KEY_SIZE];
                let length = bs58::decode(key_data).into(&mut array[..]).map_err(|err| {
                    Self::Err::InvalidData {
                        error_message: err.to_string(),
                    }
                })?;
                if length != SECP256K1_SECRET_KEY_SIZE {
                    return Err(Self::Err::InvalidLength {
                        expected_length: SECP256K1_SECRET_KEY_SIZE,
                        received_length: length,
                    });
                }
                Ok(Self::SECP256K1(
                    libsecp256k1::SecretKey::parse_slice(&array).map_err(|err| {
                        Self::Err::InvalidData {
                            error_message: err.to_string(),
                        }
                    })?,
                ))
            }
        }
    }
}

impl serde::Serialize for SecretKey {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        let data = match self {
            SecretKey::ED25519(secret_key) => bs58::encode(&secret_key.0[..]).into_string(),
            SecretKey::SECP256K1(secret_key) => {
                bs58::encode(&secret_key.serialize()[..]).into_string()
            }
        };
        serializer.serialize_str(&format!("{}:{}", self.key_type(), data))
    }
}

impl<'de> serde::Deserialize<'de> for SecretKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

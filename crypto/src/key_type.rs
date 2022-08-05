use {
    core::{
        fmt::{Display, Formatter},
        str::FromStr,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum KeyType {
    ED25519 = 0,
    SECP256K1 = 1,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                KeyType::ED25519 => "ed25519",
                KeyType::SECP256K1 => "secp256k1",
            },
        )
    }
}

impl FromStr for KeyType {
    type Err = crate::ParseKeyTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lowercase_key_type = value.to_ascii_lowercase();
        match lowercase_key_type.as_str() {
            "ed25519" => Ok(KeyType::ED25519),
            "secp256k1" => Ok(KeyType::SECP256K1),
            _ => Err(Self::Err::UnknownKeyType {
                unknown_key_type: lowercase_key_type,
            }),
        }
    }
}

impl TryFrom<u8> for KeyType {
    type Error = crate::errors::ParseKeyTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyType::ED25519),
            1 => Ok(KeyType::SECP256K1),
            unknown_key_type => Err(Self::Error::UnknownKeyType {
                unknown_key_type: unknown_key_type.to_string(),
            }),
        }
    }
}

pub fn split_key_type_data(
    value: &str,
) -> Result<(KeyType, &str), crate::errors::ParseKeyTypeError> {
    if let Some(idx) = value.find(':') {
        let (prefix, key_data) = value.split_at(idx);
        Ok((KeyType::from_str(prefix)?, &key_data[1..]))
    } else {
        // If there is no prefix then we Default to ED25519.
        Ok((KeyType::ED25519, value))
    }
}

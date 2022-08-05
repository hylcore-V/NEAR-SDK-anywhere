use {
    super::{Secp256K1PublicKey, SECP256K1_SIGNATURE_LENGTH},
    crate::ParseSignatureError,
    borsh::{BorshDeserialize, BorshSerialize},
    core::fmt::{Debug, Formatter},
    primitive_types::U256,
};

const SECP256K1_N: U256 = U256([
    0xbfd25e8cd0364141,
    0xbaaedce6af48a03b,
    0xfffffffffffffffe,
    0xffffffffffffffff,
]);

// Half of SECP256K1_N + 1.
const SECP256K1_N_HALF_ONE: U256 = U256([
    0xdfe92f46681b20a1,
    0x5d576e7357a4501d,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Secp256K1Signature(pub [u8; SECP256K1_SIGNATURE_LENGTH]);

impl Secp256K1Signature {
    pub fn check_signature_values(&self, reject_upper: bool) -> bool {
        let mut r_bytes = [0u8; 32];
        r_bytes.copy_from_slice(&self.0[0..32]);
        let r = U256::from(r_bytes);

        let mut s_bytes = [0u8; 32];
        s_bytes.copy_from_slice(&self.0[32..64]);
        let s = U256::from(s_bytes);

        let s_check = if reject_upper {
            // Reject upper range of s values (ECDSA malleability)
            SECP256K1_N_HALF_ONE
        } else {
            SECP256K1_N
        };

        r < SECP256K1_N && s < s_check
    }

    pub fn recover(
        &self,
        data: &[u8],
        recovery_id: u8,
    ) -> Result<Secp256K1PublicKey, ParseSignatureError> {
        let message = libsecp256k1::Message::parse_slice(data).map_err(|_| {
            ParseSignatureError::InvalidData {
                error_message: "failed to parse message".to_string(),
            }
        })?;
        let recovery_id = libsecp256k1::RecoveryId::parse(recovery_id).map_err(|_| {
            ParseSignatureError::InvalidData {
                error_message: "failed to parse recovery_id".to_string(),
            }
        })?;
        let signature = libsecp256k1::Signature::parse_standard_slice(&self.0).map_err(|_| {
            ParseSignatureError::InvalidData {
                error_message: "failed to parse signature".to_string(),
            }
        })?;
        let secp256k1_key =
            libsecp256k1::recover(&message, &signature, &recovery_id).map_err(|_| {
                ParseSignatureError::InvalidData {
                    error_message: "failed to recover data".to_string(),
                }
            })?;
        Ok(Secp256K1PublicKey::from(
            &secp256k1_key.serialize()[1..SECP256K1_SIGNATURE_LENGTH],
        ))
    }

    pub fn verify(&self, data: &[u8], public_key: &[u8]) -> Result<(), ParseSignatureError> {
        let public_key = libsecp256k1::PublicKey::parse_slice(&public_key, None).map_err(|_| {
            ParseSignatureError::InvalidData {
                error_message: "failed to parse public key".to_string(),
            }
        })?;
        let message = libsecp256k1::Message::parse_slice(data).map_err(|_| {
            ParseSignatureError::InvalidData {
                error_message: "failed to parse message".to_string(),
            }
        })?;

        let signature = libsecp256k1::Signature::parse_standard_slice(&self.0).map_err(|_| {
            ParseSignatureError::InvalidData {
                error_message: "failed to parse signature".to_string(),
            }
        })?;

        libsecp256k1::verify(&message, &signature, &public_key);
        Ok(())
    }
}

impl From<[u8; 65]> for Secp256K1Signature {
    fn from(data: [u8; 65]) -> Self {
        Self(data)
    }
}

impl TryFrom<&[u8]> for Secp256K1Signature {
    type Error = crate::ParseSignatureError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 65 {
            return Err(Self::Error::InvalidLength {
                expected_length: SECP256K1_SIGNATURE_LENGTH,
                received_length: data.len(),
            });
        }
        let mut signature = Self([0; SECP256K1_SIGNATURE_LENGTH]);
        signature.0.copy_from_slice(data);
        Ok(signature)
    }
}

impl Debug for Secp256K1Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", bs58::encode(&self.0.to_vec()).into_string())
    }
}

impl From<Secp256K1Signature> for [u8; SECP256K1_SIGNATURE_LENGTH] {
    fn from(sig: Secp256K1Signature) -> [u8; SECP256K1_SIGNATURE_LENGTH] {
        sig.0
    }
}

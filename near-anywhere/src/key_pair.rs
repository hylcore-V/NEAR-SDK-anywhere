use {
    crate::crypto::{KeyType, ParseKeyError, PublicKey, SecretKey, Signature},
    core::{fmt::Display, str::FromStr},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyPair {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl KeyPair {
    pub fn from_random(key_type: KeyType) -> Self {
        match key_type {
            KeyType::ED25519 => {
                let secret_key = SecretKey::from_random(KeyType::ED25519);
                let public_key = secret_key.public_key();

                Self {
                    secret_key,
                    public_key,
                }
            }
            KeyType::SECP256K1 => {
                let secret_key = SecretKey::from_random(KeyType::SECP256K1);
                let public_key = secret_key.public_key();

                Self {
                    secret_key,
                    public_key,
                }
            }
        }
    }

    pub fn from_secret_key(secret_key_data: &str) -> Result<Self, ParseKeyError> {
        let secret_key = SecretKey::from_str(secret_key_data)?;
        let public_key = secret_key.public_key();

        Ok(Self {
            secret_key,
            public_key,
        })
    }
}

impl KeyPair {
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.secret_key.sign(message)
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn verify(&self, message: &[u8], signature: Signature) -> bool {
        signature.verify(message, &self.public_key)
    }
}

impl Display for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.secret_key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sign_and_verify() {
        let key_pair = KeyPair::from_secret_key("ed25519:2LRHYvi3uHHsADkF8HFRyZXJX3BG7HuQpuHw1cWcYSrivxNo5y76vkPA4ezKixS3jQ7e2zCfi4zfXbNAP72j2Ntk").expect("Failed to create key pair");
        let public_key = key_pair.public_key();
        assert_eq!(
            public_key.to_string(),
            "ed25519:G9upgDmY9DPYvkxZrR52Foh7g351TKee1E4BJdCLfZaU"
        );
        use sha2::Digest;
        let data = sha2::Sha256::digest(b"hello world").to_vec();
        let signature = key_pair.sign(&data);
        assert!(signature.verify(&data, &public_key));
    }
}

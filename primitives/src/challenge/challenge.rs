use {
    super::ChallengeBody,
    crate::hash::CryptoHash,
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::Signature,
    near_primitives_core::{hash::hash, types::AccountId},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
#[borsh_init(init)]
pub struct Challenge {
    pub body: ChallengeBody,
    pub account_id: AccountId,
    pub signature: Signature,

    #[borsh_skip]
    pub hash: CryptoHash,
}

impl Challenge {
    pub fn init(&mut self) {
        self.hash = hash(&self.body.try_to_vec().expect("Failed to serialize"));
    }
    // TODO: we dont need this
    // pub fn produce(body: ChallengeBody, signer: &dyn ValidatorSigner) -> Self {
    //     let (hash, signature) = signer.sign_challenge(&body);
    //     Self {
    //         body,
    //         account_id: signer.validator_id().clone(),
    //         signature,
    //         hash,
    //     }
    // }
}

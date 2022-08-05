use {
    super::{ActionReceipt, ReceiptEnum},
    crate::actions::{Action, TransferAction},
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::{KeyType, PublicKey},
    near_primitives_core::{
        hash::CryptoHash,
        types::{AccountId, Balance},
    },
    std::borrow::Borrow,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Receipt {
    /// An issuer account_id of a particular receipt.
    /// `predecessor_id` could be either `Transaction` `signer_id` or intermediate contract's `account_id`.
    pub predecessor_id: AccountId,
    /// `receiver_id` is a receipt destination.
    pub receiver_id: AccountId,
    /// An unique id for the receipt
    pub receipt_id: CryptoHash,
    /// A receipt type
    pub receipt: ReceiptEnum,
}

impl Borrow<CryptoHash> for Receipt {
    fn borrow(&self) -> &CryptoHash {
        &self.receipt_id
    }
}

impl Receipt {
    /// It's not a content hash, but receipt_id is unique.
    pub fn get_hash(&self) -> CryptoHash {
        self.receipt_id
    }

    /// Generates a receipt with a transfer from system for a given balance without a receipt_id.
    /// This should be used for token refunds instead of gas refunds. It doesn't refund the
    /// allowance of the access key. For gas refunds use `new_gas_refund`.
    pub fn new_balance_refund(receiver_id: &AccountId, refund: Balance) -> Self {
        Receipt {
            predecessor_id: "system".parse().unwrap(),
            receiver_id: receiver_id.clone(),
            receipt_id: CryptoHash::default(),

            receipt: ReceiptEnum::Action(ActionReceipt {
                signer_id: "system".parse().unwrap(),
                signer_public_key: PublicKey::empty(KeyType::ED25519),
                gas_price: 0,
                output_data_receivers: vec![],
                input_data_ids: vec![],
                actions: vec![Action::Transfer(TransferAction { deposit: refund })],
            }),
        }
    }

    /// Generates a receipt with a transfer action from system for a given balance without a
    /// receipt_id. It contains `signer_id` and `signer_public_key` to indicate this is a gas
    /// refund. The execution of this receipt will try to refund the allowance of the
    /// access key with the given public key.
    /// NOTE: The access key may be replaced by the owner, so the execution can't rely that the
    /// access key is the same and it should use best effort for the refund.
    pub fn new_gas_refund(
        receiver_id: &AccountId,
        refund: Balance,
        signer_public_key: PublicKey,
    ) -> Self {
        Receipt {
            predecessor_id: "system".parse().unwrap(),
            receiver_id: receiver_id.clone(),
            receipt_id: CryptoHash::default(),

            receipt: ReceiptEnum::Action(ActionReceipt {
                signer_id: receiver_id.clone(),
                signer_public_key,
                gas_price: 0,
                output_data_receivers: vec![],
                input_data_ids: vec![],
                actions: vec![Action::Transfer(TransferAction { deposit: refund })],
            }),
        }
    }
}

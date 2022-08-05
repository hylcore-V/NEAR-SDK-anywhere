use {
    super::DataReceiverView,
    crate::{
        actions::ActionView,
        serialize::{option_base64_format, u128_dec_format},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    near_anywhere_crypto::PublicKey,
    near_primitives_core::{
        hash::CryptoHash,
        types::{AccountId, Balance},
    },
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ReceiptEnumView {
    Action {
        signer_id: AccountId,
        signer_public_key: PublicKey,
        #[serde(with = "u128_dec_format")]
        gas_price: Balance,
        output_data_receivers: Vec<DataReceiverView>,
        input_data_ids: Vec<CryptoHash>,
        actions: Vec<ActionView>,
    },
    Data {
        data_id: CryptoHash,
        #[serde(with = "option_base64_format")]
        data: Option<Vec<u8>>,
    },
}

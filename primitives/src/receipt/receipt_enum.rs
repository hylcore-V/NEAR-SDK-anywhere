use {
    super::{ActionReceipt, DataReceipt},
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ReceiptEnum {
    Action(ActionReceipt),
    Data(DataReceipt),
}

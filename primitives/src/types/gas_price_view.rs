use crate::{serialize::u128_dec_format, types::Balance};

#[derive(Serialize, Deserialize, Debug)]
pub struct GasPriceView {
    #[serde(with = "u128_dec_format")]
    pub gas_price: Balance,
}

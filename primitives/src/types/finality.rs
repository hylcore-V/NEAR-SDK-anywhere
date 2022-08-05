use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Finality {
    #[serde(rename = "optimistic")]
    None,
    #[serde(rename = "near-final")]
    DoomSlug,
    #[serde(rename = "final")]
    Final,
}

impl Default for Finality {
    fn default() -> Self {
        Finality::Final
    }
}

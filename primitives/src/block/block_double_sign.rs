use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub struct BlockDoubleSign {
    pub left_block_header: Vec<u8>,
    pub right_block_header: Vec<u8>,
}

impl std::fmt::Display for BlockDoubleSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

use {
    crate::{
        block::BlockDoubleSign,
        sharding::{ChunkProofs, ChunkState},
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub enum ChallengeBody {
    BlockDoubleSign(BlockDoubleSign),
    ChunkProofs(ChunkProofs),
    ChunkState(ChunkState),
}

use {
    super::{
        CurrentEpochValidatorInfo,
        NextEpochValidatorInfo,
        ValidatorKickoutView,
        ValidatorStakeView,
    },
    crate::types::{BlockHeight, EpochHeight},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EpochValidatorInfo {
    /// Validators for the current epoch
    pub current_validators: Vec<CurrentEpochValidatorInfo>,
    /// Validators for the next epoch
    pub next_validators: Vec<NextEpochValidatorInfo>,
    /// Fishermen for the current epoch
    pub current_fishermen: Vec<ValidatorStakeView>,
    /// Fishermen for the next epoch
    pub next_fishermen: Vec<ValidatorStakeView>,
    /// Proposals in the current epoch
    pub current_proposals: Vec<ValidatorStakeView>,
    /// Kickout in the previous epoch
    pub prev_epoch_kickout: Vec<ValidatorKickoutView>,
    /// Epoch start block height
    pub epoch_start_height: BlockHeight,
    /// Epoch height
    pub epoch_height: EpochHeight,
}

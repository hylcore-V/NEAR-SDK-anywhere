mod challenge;
mod challenge_body;

use crate::network::SlashedValidator;
pub use {challenge::Challenge, challenge_body::ChallengeBody};

/// Result of checking challenge, contains which accounts to slash.
/// If challenge is invalid this is sender, otherwise author of chunk (and possibly other participants that signed invalid blocks).
pub type ChallengesResult = Vec<SlashedValidator>;
pub type Challenges = Vec<Challenge>;

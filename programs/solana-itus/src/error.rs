use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Invalid epoch ID")]
    InvalidEpochId,
    #[msg("Epoch not finished")]
    EpochNotFinished,
    #[msg("No rewards to distribute")]
    NoRewardsToDistribute,
    #[msg("Rewards already claimed")]
    RewardsAlreadyClaimed,
    #[msg("No winner for this epoch")]
    NoWinnerForEpoch,
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
}

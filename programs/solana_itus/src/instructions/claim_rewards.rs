use crate::state::{State, UserRewards};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use crate::error::MyError;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut, has_one = user)]
    pub user_rewards: Account<'info, UserRewards>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reward_destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, token::Token>,
}

pub fn handler(ctx: Context<ClaimRewards>, epoch_id: u64) -> Result<()> {
    // Borrow mutable references to `state` and `user_rewards`
    let state = &mut ctx.accounts.state;
    let user_rewards = &mut ctx.accounts.user_rewards;

    // Ensure the epoch ID is valid
    require!(epoch_id < state.epoch_id, MyError::InvalidEpochId);

    // Calculate the reward for the user
    let reward_amount = user_rewards.claimable(epoch_id);

    // Ensure there are rewards to claim
    require!(reward_amount > 0, MyError::NoRewardsToClaim);

    // Drop the mutable references before creating the CPI context
    drop(state);
    drop(user_rewards);

    // Transfer the reward to the user's reward destination
    token::transfer(ctx.accounts.into_transfer_context(), reward_amount)?;

    // Re-borrow mutable references to update user's rewards
    let user_rewards = &mut ctx.accounts.user_rewards;
    user_rewards.rewards_claimed(epoch_id, reward_amount)?;

    Ok(())
}

impl<'info> ClaimRewards<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.rewards_account.to_account_info(),
                to: self.reward_destination.to_account_info(),
                authority: self.user.to_account_info(), // Fixed the authority field
            },
        )
    }
}

use crate::state::{State, UserRewards};
use crate::error::MyError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

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
    let state = &mut ctx.accounts.state;
    let user_rewards = &mut ctx.accounts.user_rewards;

    require!(epoch_id < state.epoch_id, MyError::InvalidEpochId);
    let reward_amount = user_rewards.claimable(epoch_id);
    require!(reward_amount > 0, MyError::NoRewardsToClaim);

    let _ = state;  // Explicitly ignore state
    let _ = user_rewards;  // Explicitly ignore user_rewards

    token::transfer(ctx.accounts.into_transfer_context(), reward_amount)?;

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

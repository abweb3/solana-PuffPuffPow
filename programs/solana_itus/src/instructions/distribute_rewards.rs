use crate::error::MyError;
use crate::state::State;
use crate::utils::calculate_epoch_id;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reward_destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DistributeRewards>) -> Result<()> {
    // Borrow mutable reference to `state`
    let state = &mut ctx.accounts.state;
    let current_epoch_id = calculate_epoch_id(state.last_epoch_timestamp, state.epoch_duration);

    // Calculate the total reward amount for the current epoch
    let total_reward = state.calculate_total_reward(current_epoch_id)?;

    // Ensure there are rewards to distribute
    require!(total_reward > 0, MyError::NoRewardsToDistribute);

    // Drop the mutable reference before creating the CPI context
    drop(state);

    // Transfer the rewards to the reward destination account
    token::transfer(ctx.accounts.into_transfer_context(), total_reward)?;

    // Re-borrow mutable reference to update the state with the distributed rewards
    let state = &mut ctx.accounts.state;
    state.rewards_distributed(current_epoch_id, total_reward)?;

    Ok(())
}

impl<'info> DistributeRewards<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.rewards_account.to_account_info(),
                to: self.reward_destination.to_account_info(),
                authority: self.state.to_account_info(),
            },
        )
    }
}

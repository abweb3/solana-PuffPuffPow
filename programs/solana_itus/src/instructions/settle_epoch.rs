use crate::state::State;
use crate::utils::calculate_epoch_id;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SettleEpoch<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub bottom_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub top_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reward_destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, token::Token>,
}

pub fn handler(ctx: Context<SettleEpoch>) -> Result<()> {
    // Borrow mutable reference to `state`
    let state = &mut ctx.accounts.state;
    let current_epoch_id = calculate_epoch_id(state.last_epoch_timestamp, state.epoch_duration);

    // Settle the epoch by calculating the market cap and rewards
    state.settle_epoch(current_epoch_id)?;

    // Drop the mutable reference before creating the CPI context
    drop(state);

    // Transfer the rewards to the reward destination account
    token::transfer(ctx.accounts.into_transfer_context(), ctx.accounts.rewards_account.amount)?;

    // Re-borrow mutable reference to update the state with the settled epoch data
    let state = &mut ctx.accounts.state;
    state.update_epoch_duration()?; // Adjusted to use an existing function

    Ok(())
}

impl<'info> SettleEpoch<'info> {
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

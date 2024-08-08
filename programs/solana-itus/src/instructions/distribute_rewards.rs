use crate::state::State;
use crate::utils::calculate_epoch_id;
use crate::error::MyError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, Token};

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
    let total_reward;
    let current_epoch_id;
    {
        let state = &mut ctx.accounts.state;
        current_epoch_id = calculate_epoch_id(state.last_epoch_timestamp, state.epoch_duration);
        total_reward = state.calculate_total_reward(current_epoch_id)?;
        require!(total_reward > 0, MyError::NoRewardsToDistribute);
    }

    token::transfer(ctx.accounts.into_transfer_context(), total_reward)?;

    {
        let state = &mut ctx.accounts.state;
        state.rewards_distributed(current_epoch_id, total_reward)?;
    }

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

use crate::state::State;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + State::LEN)]
    pub state: Account<'info, State>,
    pub bottom_token: Account<'info, Mint>,
    pub top_token: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, epoch_duration: i64, max_reward: u64) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.epoch_duration = epoch_duration;
    state.max_reward = max_reward;
    state.last_epoch_timestamp = Clock::get()?.unix_timestamp;
    state.last_market_cap = 0;
    state.bottom_token = ctx.accounts.bottom_token.key();
    state.top_token = ctx.accounts.top_token.key();
    Ok(())
}

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, Token};

#[derive(Accounts)]
pub struct InitializePools<'info> {
    #[account(mut)]
    pub bottom_token: Account<'info, Mint>,
    #[account(mut)]
    pub top_token: Account<'info, Mint>,
    #[account(init, payer = user, token::mint = bottom_token, token::authority = user)]
    pub lp_bottom_top: Account<'info, TokenAccount>,
    #[account(init, payer = user, token::mint = bottom_token, token::authority = user)]
    pub lp_bottom_sol: Account<'info, TokenAccount>,
    #[account(init, payer = user, token::mint = top_token, token::authority = user)]
    pub lp_top_sol: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializePools>) -> Result<()> {
    let bottom_token_amount = (69_420_000_000_000u64 * 42) / 100;
    let top_token_amount = (69_420_000_000_000u64 * 42) / 100;

    // Transfer tokens to liquidity pool accounts
    token::transfer(
        ctx.accounts.into_transfer_context(
            ctx.accounts.user.clone(),
            ctx.accounts.lp_bottom_top.to_account_info(),
        ),
        bottom_token_amount,
    )?;
    token::transfer(
        ctx.accounts.into_transfer_context(
            ctx.accounts.user.clone(),
            ctx.accounts.lp_bottom_sol.to_account_info(),
        ),
        bottom_token_amount,
    )?;
    token::transfer(
        ctx.accounts.into_transfer_context(
            ctx.accounts.user.clone(),
            ctx.accounts.lp_top_sol.to_account_info(),
        ),
        top_token_amount,
    )?;

    Ok(())
}

impl<'info> InitializePools<'info> {
    fn into_transfer_context(
        &self,
        authority: Signer<'info>,
        to: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.bottom_token.to_account_info(),
                to,
                authority: authority.to_account_info(),
            },
        )
    }
}

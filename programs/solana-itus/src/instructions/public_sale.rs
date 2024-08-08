use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct PublicSale<'info> {
    #[account(mut)]
    pub bottom_token: Account<'info, Mint>,
    #[account(mut)]
    pub top_token: Account<'info, Mint>,
    #[account(mut)]
    pub bottom_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub top_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub lp_bottom_top: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_bottom_sol: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_top_sol: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<PublicSale>, _amount: u64) -> Result<()> {
    let bottom_token_amount = (69_420_000_000_000u64 * 42) / 100;
    let top_token_amount = (69_420_000_000_000u64 * 42) / 100;

    // Transfer tokens to public sale account
    token::transfer(
        ctx.accounts.into_transfer_context_lp_bottom_top(),
        bottom_token_amount,
    )?;
    token::transfer(
        ctx.accounts.into_transfer_context_lp_bottom_sol(),
        bottom_token_amount,
    )?;
    token::transfer(
        ctx.accounts.into_transfer_context_lp_top_sol(),
        top_token_amount,
    )?;

    Ok(())
}

impl<'info> PublicSale<'info> {
    fn into_transfer_context_lp_bottom_top(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.bottom_token_account.to_account_info(),
                to: self.lp_bottom_top.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_context_lp_bottom_sol(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.bottom_token_account.to_account_info(),
                to: self.lp_bottom_sol.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    fn into_transfer_context_lp_top_sol(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.top_token_account.to_account_info(),
                to: self.lp_top_sol.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }
}

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token};

#[derive(Accounts)]
pub struct InitializeTokens<'info> {
    #[account(init, payer = user, mint::decimals = 9, mint::authority = mint_authority)]
    pub bottom_token: Account<'info, Mint>,
    #[account(init, payer = user, mint::decimals = 9, mint::authority = mint_authority)]
    pub top_token: Account<'info, Mint>,
    #[account(seeds = [b"mint-authority"], bump)]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeTokens>) -> Result<()> {
    let fixed_supply = 69_420_000_000_000; // 69,420,000.00 tokens with 9 decimals

    // Mint the fixed supply of tokens to the user
    token::mint_to(
        ctx.accounts
            .into_mint_to_context(ctx.accounts.mint_authority.clone()),
        fixed_supply,
    )?;
    token::mint_to(
        ctx.accounts
            .into_mint_to_context(ctx.accounts.mint_authority.clone()),
        fixed_supply,
    )?;

    Ok(())
}

impl<'info> InitializeTokens<'info> {
    fn into_mint_to_context(
        &self,
        authority: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.bottom_token.to_account_info(),
                to: self.user.to_account_info(),
                authority,
            },
        )
    }
}

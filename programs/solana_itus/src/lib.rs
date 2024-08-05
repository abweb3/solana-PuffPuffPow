use anchor_lang::prelude::*;
use instructions::{
    claim_rewards::ClaimRewards, distribute_rewards::DistributeRewards, initialize::Initialize,
    initialize_pools::InitializePools, initialize_tokens::InitializeTokens,
    public_sale::PublicSale, settle_epoch::SettleEpoch,
};

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("FWnGsp5dSMW91H8ap8zc4BPjqfTN4yMg9PYxedw3mZGy");


pub mod solana_itus {
    use instructions::{claim_rewards, distribute_rewards, initialize, initialize_pools, initialize_tokens, public_sale, settle_epoch};

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        epoch_duration: i64,
        max_reward: u64,
    ) -> Result<()> {
        initialize::handler(ctx, epoch_duration, max_reward)
    }

    pub fn initialize_tokens(ctx: Context<InitializeTokens>) -> Result<()> {
        initialize_tokens::handler(ctx)
    }

    pub fn initialize_pools(ctx: Context<InitializePools>) -> Result<()> {
        initialize_pools::handler(ctx)
    }

    pub fn public_sale(ctx: Context<PublicSale>, amount: u64) -> Result<()> {
        public_sale::handler(ctx, amount)
    }

    pub fn settle_epoch(ctx: Context<SettleEpoch>) -> Result<()> {
        settle_epoch::handler(ctx)
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        distribute_rewards::handler(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, epoch_id: u64) -> Result<()> {
        claim_rewards::handler(ctx, epoch_id)
    }
}

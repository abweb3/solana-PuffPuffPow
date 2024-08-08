use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub epoch_duration: i64,
    pub max_reward: u64,
    pub last_epoch_timestamp: i64,
    pub last_market_cap: u64,
    pub bottom_token: Pubkey,
    pub top_token: Pubkey,
    pub last_epoch_winner: Winner,
    pub epoch_rewards: Vec<EpochReward>, // Changed from HashMap to Vec
    pub has_claimed_reward: Vec<ClaimedReward>, // Changed from HashMap to Vec
    pub rewards_account: Pubkey,
    pub epoch_id: u64,
    pub daily_votes: u64,
    pub weekly_votes: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct EpochReward {
    pub epoch_id: u64,
    pub reward: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct ClaimedReward {
    pub epoch_id: u64,
    pub user: Pubkey,
    pub claimed: bool,
}

impl State {
    pub const LEN: usize =
        8 + 8 + 8 + 8 + 32 + 32 + 1 + (8 + 8) * 10 + (8 + 32) * 10 + 32 + 8 + 8 + 8;

    pub fn calculate_total_reward(&mut self, _epoch_id: u64) -> Result<u64> {
        let total_reward = self.last_market_cap / 100;
        Ok(total_reward)
    }

    pub fn rewards_distributed(&mut self, epoch_id: u64, total_reward: u64) -> Result<()> {
        self.epoch_rewards.push(EpochReward {
            epoch_id,
            reward: total_reward,
        });
        Ok(())
    }

    pub fn settle_epoch(&mut self, epoch_id: u64) -> Result<()> {
        let market_cap_increase = 1000;
        self.last_market_cap += market_cap_increase;

        if self.last_market_cap % 2 == 0 {
            self.last_epoch_winner = Winner::Bottom;
        } else {
            self.last_epoch_winner = Winner::Top;
        }

        self.epoch_id = epoch_id;
        Ok(())
    }

    pub fn update_epoch_duration(&mut self) -> Result<()> {
        if self.daily_votes > self.weekly_votes {
            self.epoch_duration = 86400; // 1 day in seconds
        } else {
            self.epoch_duration = 604800; // 1 week in seconds
        }
        Ok(())
    }

    pub fn vote_for_epoch_duration(&mut self, user_vote: EpochDurationVote) -> Result<()> {
        match user_vote {
            EpochDurationVote::Daily => self.daily_votes += 1,
            EpochDurationVote::Weekly => self.weekly_votes += 1,
        }
        self.update_epoch_duration()
    }
}

#[account]
pub struct UserRewards {
    pub user: Pubkey,
    pub rewards: Vec<EpochReward>, // Changed from HashMap to Vec
}

impl UserRewards {
    pub fn claimable(&self, epoch_id: u64) -> u64 {
        self.rewards
            .iter()
            .find(|reward| reward.epoch_id == epoch_id)
            .map(|reward| reward.reward)
            .unwrap_or(0)
    }

    pub fn rewards_claimed(&mut self, epoch_id: u64, amount: u64) -> Result<()> {
        if let Some(reward) = self
            .rewards
            .iter_mut()
            .find(|reward| reward.epoch_id == epoch_id)
        {
            if reward.reward >= amount {
                reward.reward -= amount;
                Ok(())
            } else {
                Err(ProgramError::InsufficientFunds.into())
            }
        } else {
            Err(ProgramError::InvalidArgument.into())
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Winner {
    None,
    Bottom,
    Top,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EpochDurationVote {
    Daily,
    Weekly,
}

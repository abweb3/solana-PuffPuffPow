use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

pub fn calculate_epoch_id(last_epoch_timestamp: i64, epoch_duration: i64) -> u64 {
    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    ((current_timestamp - last_epoch_timestamp) / epoch_duration) as u64
}

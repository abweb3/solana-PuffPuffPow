use anchor_lang::solana_program::clock::Epoch;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::TokenAccount;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::clock::Clock;
use std::convert::TryInto;
use std::ptr::addr_of_mut;
use std::str::FromStr;

const RAYDIUM_LP_ADDRESS: &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"; // Replace with actual Raydium LP address

// Function to get the current market cap of the bottom and top tokens
pub fn get_current_market_cap(bottom_token: Pubkey, top_token: Pubkey) -> anchor_lang::Result<u64> {
    let lp_account = get_lp_account(RAYDIUM_LP_ADDRESS)?;

    // Get the reserves of bottom and top tokens from the LP account
    let (reserve_bottom, reserve_top) = get_reserves_from_lp(lp_account)?;

    // Calculate the price of bottom and top tokens
    let price_bottom = reserve_top as f64 / reserve_bottom as f64;
    let price_top = reserve_bottom as f64 / reserve_top as f64;

    let bottom_supply = get_token_supply(bottom_token)?;
    let top_supply = get_token_supply(top_token)?;

    let market_cap_bottom = (bottom_supply as f64 * price_bottom) as u64;
    let market_cap_top = (top_supply as f64 * price_top) as u64;

    Ok(market_cap_bottom + market_cap_top)
}

// Function to get the LP account info given an LP address
fn get_lp_account(lp_address: &str) -> anchor_lang::Result<AccountInfo<'static>> {
    static mut LAMPORTS: u64 = 0;
    static mut DATA: [u8; 0] = [];
    static mut LEAKED_LP_PUBKEY: Option<Pubkey> = None;

    let lp_pubkey = Pubkey::from_str(lp_address).map_err(|_| ProgramError::InvalidArgument)?;

    unsafe {
        LEAKED_LP_PUBKEY = Some(lp_pubkey);

        let account_info = AccountInfo::new(
            LEAKED_LP_PUBKEY.as_ref().unwrap(),
            false,
            false,
            addr_of_mut!(LAMPORTS),
            addr_of_mut!(DATA),
            &Pubkey::default(),
            false,
            Epoch::default(),
        );

        Ok(account_info)
    }
}

// Function to get the reserves of bottom and top tokens from the LP account
fn get_reserves_from_lp(lp_account: AccountInfo<'_>) -> anchor_lang::Result<(u64, u64)> {
    let lp_data: std::cell::Ref<&mut [u8]> = lp_account.try_borrow_data()?;
    let (reserve_bottom, reserve_top) = (
        u64::from_le_bytes(
            lp_data[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ),
        u64::from_le_bytes(
            lp_data[8..16]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ),
    );
    Ok((reserve_bottom, reserve_top))
}

// Function to get the total supply of a given token
fn get_token_supply(token_pubkey: Pubkey) -> anchor_lang::Result<u64> {
    static mut LAMPORTS: u64 = 0;
    static mut DATA: [u8; 0] = [];
    static mut LEAKED_TOKEN_PUBKEY: Option<Pubkey> = None;

    unsafe {
        LEAKED_TOKEN_PUBKEY = Some(token_pubkey);

        let binding = Pubkey::default();
        let account_info = AccountInfo::new(
            LEAKED_TOKEN_PUBKEY.as_ref().unwrap(),
            false,
            false,
            addr_of_mut!(LAMPORTS),
            addr_of_mut!(DATA),
            &binding,
            false,
            Epoch::default(),
        );

        let token_account = TokenAccount::try_deserialize(&mut &account_info.try_borrow_data()?[..])?;
        Ok(token_account.amount)
    }
}

// Function to calculate the current epoch ID based on the last epoch timestamp and epoch duration
pub fn calculate_epoch_id(last_epoch_timestamp: i64, epoch_duration: i64) -> u64 {
    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    ((current_timestamp - last_epoch_timestamp) / epoch_duration) as u64
}

use anchor_lang::prelude::*;
use pyth_sdk_solana::{load_price_feed_from_account_info};
use std::str::FromStr;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("6S31eyqebbTQP1FNWGymhnZMnu837og3d3GVrTs333PM");

const BTC_USDC_FEED: &str = "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J";
const STALENESS_THRESHOLD: u64 = 60; // in seconds

#[program]
mod pyth_contract {
    use super::*;
    pub fn fetch_btc_price(ctx: Context<FetchBitcoinPrice>) -> Result<()> {
        // 1-Fetch latest price
        let price_account_info = &ctx.accounts.price_feed;
        let price_feed = load_price_feed_from_account_info(price_account_info).unwrap();
        let current_timestamp = Clock::get()?.unix_timestamp;
        let current_price = price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

        // 2-Format display values
        let display_price = u64::try_from(current_price.price).unwrap() / 10u64.pow(u32::try_from(-current_price.expo).unwrap());
        let display_confidence = u64::try_from(current_price.conf).unwrap() / 10u64.pow(u32::try_from(-current_price.expo).unwrap());

        // 3-Log result
        msg!("BTC/USD price: ({} +- {})", display_price, display_confidence);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchBitcoinPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = Pubkey::from_str(BTC_USDC_FEED).unwrap() @ FeedError::InvalidPriceFeed)]
    pub price_feed: AccountInfo<'info>,
}

#[error_code]
pub enum FeedError {
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
}

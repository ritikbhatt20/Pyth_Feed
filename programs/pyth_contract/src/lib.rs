use anchor_lang::prelude::*;
use pyth_sdk_solana::state::SolanaPriceAccount;
use std::str::FromStr;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2DqzEj6MjKGpgNZ9BhsfXi13owfHpaqxFTxYQyrXjYVR");

const SOL_USDC_FEED: &str = "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J";

#[program]
mod pyth_contract {
    use super::*;
    pub fn fetch_btc_price(ctx: Context<FetchBitcoinPrice>) -> Result<()> {
        // Fetch the latest price
        let price_in_dollars = fetch_pyth_price(&ctx.accounts.price_feed)?;

        // Log the price in dollars to Solana's program logs
        msg!("SOL/USD price: ${:.2}", price_in_dollars);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchBitcoinPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: This is safe
    #[account(address = Pubkey::from_str(SOL_USDC_FEED).unwrap() @ ErrorCode::InvalidPriceFeed)]
    pub price_feed: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
    #[msg("Price Fetch Failed")]
    PriceFetchFailed,
}

pub fn fetch_pyth_price(price_feed_info: &AccountInfo) -> Result<f64> {
    msg!("Price Feed Info: {:?}", price_feed_info);
    let price_feed = SolanaPriceAccount::account_info_to_feed(price_feed_info)
        .map_err(|_| ErrorCode::PriceFetchFailed)?;

    msg!("Price Feed: {:?}", price_feed);
    
    let price = price_feed.get_price_unchecked();

    // Convert price to dollars by adjusting with the `expo` value
    let price_in_dollars = (price.price as f64) * 10f64.powi(price.expo);
    
    Ok(price_in_dollars)
}

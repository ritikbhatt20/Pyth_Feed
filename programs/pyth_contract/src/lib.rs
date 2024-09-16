use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

declare_id!("42249qzKKgXW7MYno4xXMo9ryqzYo1jSUzFt3D4hgofe");

#[program]
pub mod pyth_contract {
    use super::*;

    pub fn sample(ctx: Context<Sample>) -> Result<()> {
        let price_update = &mut ctx.accounts.price_update;
        // get_price_no_older_than will fail if the price update is more than 30 seconds old
        let maximum_age: u64 = 30;
        // get_price_no_older_than will fail if the price update is for a different price feed.
        // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
        let feed_id: [u8; 32] = get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?;
        let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
        // Sample output:
        // The price is (7160106530699 ± 5129162301) * 10^-8
        msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);
     
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sample<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This account is verified using the Pyth SDK, no manual checks required
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

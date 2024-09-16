use anchor_lang::prelude::*;

declare_id!("42249qzKKgXW7MYno4xXMo9ryqzYo1jSUzFt3D4hgofe");

#[program]
pub mod pyth_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

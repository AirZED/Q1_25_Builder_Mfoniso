use anchor_lang::prelude::*;

mod contexts;
use contexts::*;

mod state;
use state::*;

declare_id!("6BacKD9ynDDBPq3ATnWXL7HEUwWdnKnYkueDrcnAab96");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, recieve: u64) -> Result<()> {
        //    ctx.accounts.save_escrow(seed, recieve, ctx.bumps)?;
        //    ctx.accounts.deposit_to_vault(amount)
        Ok(())
    }

    pub fn take(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        //    ctx.accounts.transfer()?;
        //    ctx.accounts.withdraw_and_close()

        Ok(())
    }

    pub fn refund(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

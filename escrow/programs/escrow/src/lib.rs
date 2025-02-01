use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

mod state;
use state::*;

mod constants;
use constants::*;

declare_id!("DjLVzRa7F44qF4SY6KJDMNxAY93u1VH7iGV2Y6ERSt4F");

#[program]
pub mod escrow {

    use super::*;

    pub fn make(ctx: Context<Make>, seeds: u64, recieve: u64, bump: u64) -> Result<()> {
        let escrow = ctx.accounts;
        escrow.init_escrow(seeds, recieve, &ctx.bumps)?;
        escrow.deposit(recieve)?;

        Ok(())
    }

    // pub fn deposit(ctx: Context<Make>, amount: u64) -> Result<()> {
    //     let escrow = &mut ctx.accounts.escrow;
    //     Ok(())
    // }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        let escrow = ctx.accounts;
        escrow._take()?;

        Ok(())
    }

    pub fn close(ctx: Context<Take>) -> Result<()> {
        let escrow = ctx.accounts;
        escrow.close()?;
        Ok(())
    }
}

use anchor_lang::prelude::*;

mod instructions;
use instruction::*;

mod state;
use state::*;



declare_id!("DjLVzRa7F44qF4SY6KJDMNxAY93u1VH7iGV2Y6ERSt4F");

#[program]
pub mod escrow_q4 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

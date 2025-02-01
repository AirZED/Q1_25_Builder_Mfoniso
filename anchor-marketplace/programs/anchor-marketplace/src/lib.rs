use anchor_lang::prelude::*;

declare_id!("26SoNrRdLXyzxe4W8nYqeUZYEftSpKy6yesFNCXrmb6d");

mod context;
mod error;
mod state;

use context::*;
use error::*;
use state::*;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)
    }

    // pub fn list(ctx: Context<List>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn delist(ctx: Context<Delist>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     Ok(())
    // }
}

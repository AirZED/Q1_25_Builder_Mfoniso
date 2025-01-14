use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Escrow {
    seed: u64,
    maker: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    bump: u8,
}

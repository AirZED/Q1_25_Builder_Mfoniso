use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketPlace {
    pub admin: Pubkey,     // 8 bytes
    pub fee: u16,          // 2 bytes
    pub bump: u8,          // 2 byte
    pub treasury_bump: u8, // 1 byte
    pub reward_bump: u8,   // 1 byte

    #[max_len(32)]
    pub name: String, // 8 bytes
}

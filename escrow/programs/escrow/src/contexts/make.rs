use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface },
};

use crate::Escrow;


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut, 
        associated_token::mint= mint_a, 
    associated_token::authority = maker,
    associated_token::token_program = token_program
    )]
    maker_ata_a:InterfaceAccount<'info, TokenAccount>,

    #[account(
    init,
    payer=maker,
    space= Escrow::INIT_SPACE,
    seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
    bump
    )]
    escrows: Account<'info, Escrow>,
  
    #[account(
    init_if_needed, 
    payer = maker,
    associated_token::mint= mint_a, 
    associated_token::authority = maker,
    associated_token::token_program = token_program
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>,
   token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,


}

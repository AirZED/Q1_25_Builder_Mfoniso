use anchor_lang::prelude::*;



#[derive(Accounts)]
pub struct Take<'info>{

    #[account(mut)]
    pub taker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,


    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", taker.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump,
    )]
    pub escrow: Account<'info, Escrow>,


    pub vault: InterfaceAccount<'info, TokenAccount>,

}
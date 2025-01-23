use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    token_interface::{Mint, TokenAccount, TokenInterface}
};



#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info>{

    #[account(mut)]
    pub maker:Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = mint_a, 
        associated_token::authority = maker,
        associated_token::token_program = token_program

    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a, 
        associated_token::authority = escrow,
        associated_token::token_program = token_program
        
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>, 
    system_program: Program<'info, System>,

}

impl<'info> Make<'info>{

pub fn init_escrow(&mut self, seed: u64, receive:u64, bump:&MakeBumps)->Result<()>{


    self.escrow.set_inner(Escrow{
       seed, 
       maker: self.maker.key(),
       mint_a: self.mint_a.key(),
       mint_b: self.mint_b.key(),
       receive,
       bump: bump.escrow,
    });

    Ok(())

}

// pub fn deposit(&mut self, amount: u64)->Result<()>{

//     let cpi_program = self.token_program.to_account_info();




// }


}




#[account]
#[derive(InitSpace)]
struct Escrow{

    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,

}


use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Mint,
    token_interface::{TokenAccount, TokenInterface},
};

use crate::state::{marketplace, Listing, MarketPlace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds=[b"marketplace", marketplace.name.as_str().as_bytes()], 
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(
        init_if_needed, 
        payer=taker, 
        associated_token::mint=maker_mint, 
        associated_token::authority=taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=maker_mint,
        associated_token::authority=listing
)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump= listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds=[b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds=[b"rewards", marketplace.key().as_ref()],
        bump=marketplace.rewards_bump,
        mint::decimals=6,
        mint::authority=marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub associatted_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

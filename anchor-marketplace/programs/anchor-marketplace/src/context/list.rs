use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{MasterEditionAccount, MetadataAccount}, token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::state::{Listing, MarketPlace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        seeds=[b"marketplace", marketplace.name.as_str().as_bytes() ],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, MarketPlace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
    pub vault: InterfaceAccount<'info, TokenAccount>,


    #[account(     
        init,
        payer=maker,
        space= Listing::INIT_SPACE,
        seeds=[b"listing", marketplace.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    pub metadata: Account<'info, MetadataAccount>,
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

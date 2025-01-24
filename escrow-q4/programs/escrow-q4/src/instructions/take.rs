use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::escrow::Escrow;
use crate::ESCROW_SEED;
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker: SystemAccount<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>, // potatoes

    pub mint_b: InterfaceAccount<'info, Mint>, //pineapples

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>, //potatoes bag

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>, //pineapples bag

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>, //pineapples bag going back to the maker

    #[account(
        mut,
        close = maker,
        // the has ones checks to be sure that the escrow as a mint_a and mint_b equals to this associated with it
        has_one = mint_b,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump,
    )]
    pub escrow: Account<'info, Escrow>, //same source of truth and state holder

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, //potatoes bag or third party

    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn withdraw_spl_from_vault(&mut self) -> Result<()> {
        let seed = &self.escrow.seed.to_le_bytes();
        let bump = self.escrow.bump;
        let maker = self.maker.key();
        let amount = self.escrow.receive;
        let decimals = self.mint_a.decimals;
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        // IN this case the signer is the escrow account
        let seeds = [ESCROW_SEED, maker.as_ref(), seed.as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer_checked(transfer_ctx, amount, decimals)?;

        Ok(())
    }

    pub fn send_spl_to_maker(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let amount = self.escrow.receive;
        let decimals = self.mint_b.decimals;

        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(transfer_ctx, amount, decimals)?;

        Ok(())
    }
    pub fn _take(&mut self) -> Result<()> {
        // call both functions together
        self.withdraw_spl_from_vault()?;
        self.send_spl_to_maker()?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        // we need to check for escrow remaining balance and add as amount
        let amount = self.vault.amount;
        let decimals = self.mint_a.decimals;

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(transfer_ctx, amount, decimals)?;

        // I believe this would have easily done the above
        self.close()?;
        Ok(())
    }
}

// for refund
// you can use a contraint= (maker.key() == escrow.maker.key())

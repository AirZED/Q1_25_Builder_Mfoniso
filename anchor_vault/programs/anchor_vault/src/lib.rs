use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("E6Z3AzBrSmLLVXUn7ptLW7sYEhTGzKakRCNi8KjB8N1u");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        space = 8+ VaultState::INIT_SPACE, 
        seeds= [b"state", user.key().as_ref()], 
        bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(seeds= [b"vault".as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.state_bump = bumps.state;
        self.state.vault_bump = bumps.vault;
       
       
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds= [b"state", user.key().as_ref()], bump=state.state_bump)]
    pub state: Account<'info, VaultState>,

    #[account(mut, seeds= [b"vault", state.key().as_ref()], bump=state.vault_bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];



        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)
    }

}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close=receiver)]
    pub data_account: Account<'info, MyData>,
    
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
 
}

#[account]
struct MyData {
    pub data: u64,
}


impl<'info> Close<'info> {
   


    }


#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub state_bump: u8,
    pub vault_bump: u8,
}

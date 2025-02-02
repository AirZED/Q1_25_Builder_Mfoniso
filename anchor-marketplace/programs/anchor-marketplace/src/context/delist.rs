use anchor_lang::prelude::*;


#[derive(Accounts)]
struct Delist <'info>{

    #[account(mut)]
    pub maker:Signer<'info>,

    


    


}
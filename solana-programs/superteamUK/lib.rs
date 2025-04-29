//form solana playground


use anchor_lang::prelude::*;

declare_id!("");

#[program]
mod wall_of_wish {
    use super::*;
    pub fn initialize(ctx: Context<MakeWishe>, title: String) -> Result<()> {
        let wish = &mut ctx.accounts.wish_account;
        wish.authority = ctx.accounts.signer.key();
        wish.title = title;
        wish.bump = ctx.bumps.wish_account;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct MakeWishe<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds  = [b"wish", signer.key().as_ref()],
        bump, 
        payer = signer, 
        space = 8 + 32 + 4 + 280
        )]
    pub wish_account: Account<'info, AWish>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AWish {
    pub authority: Pubkey,
    pub title: String,
    pub bump: u8,
}

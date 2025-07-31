use anchor_lang::prelude::*;

declare_id!("9pT2djcBqRKy82GZ8QPonPjDvchGddHbBuFEWad2H1B1");

#[program]
pub mod in_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.data_account.name = name;
        Ok(())
    }

    pub fn update(ctx: Context<UpdateAccount>, name: String) -> Result<()> {
        msg!("updating...");
        ctx.accounts.data_account.name = name;
        Ok(())
    }
}

#[account] 
pub struct AccountFormat {
    name: String
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer=payer,
        space=8+32
    )]
    pub data_account: Account<'info, AccountFormat>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateAccount<'info> {
    #[account(mut)]
    pub data_account: Account<'info, AccountFormat>,

    #[account(mut)]
    pub payer: Signer<'info>
}

use anchor_lang::prelude::*;

declare_id!("Hv886856E859RWQTVb5C5usTEseCjL4mnVY4Zz5D2jiy");

#[program]
pub mod day_21 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.acct.to_account_info().lamports();
        msg!("balance in Lamports: {}", balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don's do anything with the information
    pub acct: UncheckedAccount<'info>,
}

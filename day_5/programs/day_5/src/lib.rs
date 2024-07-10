use anchor_lang::prelude::*;

declare_id!("3Sk4JfmjMsoDwT8UZLqg2W3LQqtpc2KzQiieAUZnHv19");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("just updated");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

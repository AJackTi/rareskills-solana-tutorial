use anchor_lang::prelude::*;
use std::io::Write;

declare_id!("PUkaFKb6Kz3WTDmjibvm1DcTfcjmGjrGcUgZqUWpW9t");

#[program]
pub mod day_8 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn logs_function(_ctx: Context<Initialize>) -> Result<()> {
        std::io::stdout().write(b"Hello, world!\n").unwrap();
        Ok(())
    }

    pub fn logs_macro(_ctx: Context<Initialize>) -> Result<()> {
        msg!("{} {}", "Hello,", "world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

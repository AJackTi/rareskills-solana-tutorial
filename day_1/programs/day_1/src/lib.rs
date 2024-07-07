use anchor_lang::prelude::*;

declare_id!("F9t1oQCP1Qtxpg5vBArTMHexnM8H6pusJ5D5wzBoX6qj");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

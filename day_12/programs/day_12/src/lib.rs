use anchor_lang::prelude::*;

declare_id!("FLxgCJrnNXgN8WYwkTFEgUXJGDxAMH4qtrJ9Tto6wJT2");

#[program]
pub mod day_12 {
    use super::*;
    use anchor_lang::solana_program::sysvar::{
        instructions,
        fees::Fees,
        recent_blockhashes::RecentBlockhashes,
    };

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        // Get the Clock
        let clock = Clock::get()?;

        msg!("clock: {:?}", clock);

        // Get the Rent sysvar
        let rent_var = Rent::get()?;

        msg!("rent: {:?}", rent_var);

        // Accessing the StakeHistory sysvar
        // Create an array to store the StakeHistory account
        let arr = [ctx.accounts.stake_history.clone()];

        // Create an iterator for the array
        let accounts_iter = &mut arr.iter();

        // Get the next account info from the iterator (still StakeHistory)
        let sh_sysvar_info = next_account_info(accounts_iter)?;

        // Create a StakeHistory instance from the account info
        let stake_history = StakeHistory::from_account_info(sh_sysvar_info)?;

        msg!("stake_history: {:?}", stake_history);

        // Get instruction sysvar
        let arr = [ctx.accounts.instruction_sysvar.clone()];

        let account_info_iter = &mut arr.iter();

        let instructions_sysvar_account = next_account_info(account_info_iter)?;

        // Load the instruction details from the instruction sysvar account
        let instruction_details = instructions::load_instruction_at_checked(
            0,
            instructions_sysvar_account
        )?;

        msg!("Instruction details of this transaction: {:?}", instruction_details);
        msg!("Number is: {}", number);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>, // We create an account for the StakeHistory sysvar
    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,
    /// CHECK:
    pub instruction_sysvar: AccountInfo<'info>,
}

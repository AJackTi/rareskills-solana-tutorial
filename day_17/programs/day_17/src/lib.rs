use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("CYwoqnnQYYHyLQRdEq31MiSz8ZJG9dpMN2UfMFi1PrqT");

#[program]
pub mod day_17 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // ****************************
    // *** THIS FUNCTION IS NEW ***
    // ****************************
    pub fn set(ctx: Context<Set>, new_x: u64, new_y: u64, new_z: u64) -> Result<()> {
        // ctx.accounts.my_storage.x = new_x;
        // ctx.accounts.my_storage.y = new_y;
        // ctx.accounts.my_storage.z = new_z;

        // Instead
        let my_storage = &mut ctx.accounts.my_storage;
        my_storage.x = new_x;
        my_storage.y = new_y;
        my_storage.z = new_z;
        Ok(())
    }

    pub fn print_x(ctx: Context<PrintX>) -> Result<()> {
        let x = ctx.accounts.my_storage.x;
        msg!("The value of x is {}", x);
        Ok(())
    }

    pub fn increment_x(ctx: Context<Increment>) -> Result<()> {
        let my_storage = &mut ctx.accounts.my_storage;
        my_storage.x += 1;
        Ok(())
    }
}

// **************************
// *** THIS STRUCT IS NEW ***
// **************************
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<MyStorage>() + 8, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Accounts)]
pub struct PrintX<'info> {
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub my_storage: Account<'info, MyStorage>,
}

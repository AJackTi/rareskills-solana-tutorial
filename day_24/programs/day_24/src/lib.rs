use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("3AYs54P3hR8QhZJZ8fx5VgtB9swgczQk1qMt3F4HH68a");

const STARTING_POINTS: u32 = 10;

#[program]
pub mod day_24 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        ctx.accounts.my_storage.x = new_value;
        Ok(())
    }

    pub fn initialize_1(ctx: Context<Initialize1>) -> Result<()> {
        ctx.accounts.player.points = STARTING_POINTS;
        ctx.accounts.player.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn transfer_points_1(ctx: Context<TransferPoints1>, amount: u32) -> Result<()> {
        require!(
            ctx.accounts.from.authority == ctx.accounts.signer.key(),
            Errors::SignerIsNotAuthority
        );
        require!(ctx.accounts.from.points >= amount, Errors::InsufficientPoints);
        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;
        Ok(())
    }

    pub fn transfer_points_2(ctx: Context<TransferPoints2>, amount: u32) -> Result<()> {
        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<MyStorage>() + 8, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize1<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Player>() + 8,
        seeds = [&signer.as_ref().key().to_bytes()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

    // THIS FIELD MUST BE INCLUDED
    #[account(mut)]
    pub fren: Signer<'info>,
}

#[error_code]
pub enum Errors {
    #[msg("SignerIsNotAuthority")]
    SignerIsNotAuthority,
    #[msg("InsufficientPoints")]
    InsufficientPoints,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[derive(Accounts)]
pub struct TransferPoints1<'info> {
    #[account(mut)]
    from: Account<'info, Player>,

    #[account(mut)]
    to: Account<'info, Player>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u32)]
pub struct TransferPoints2<'info> {
    #[account(mut, has_one = authority @ Errors::SignerIsNotAuthority, constraint = from.points >= amount @ Errors::InsufficientPoints)]
    from: Account<'info, Player>,

    #[account(mut)]
    to: Account<'info, Player>,

    #[account(mut)]
    authority: Signer<'info>,
}

#[account]
pub struct Player {
    points: u32,
    authority: Pubkey,
}

use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2tLCuYpKDJtm5a82VCWHNKGD6a265KrTMHYRdEMxE8JF");

#[program]
pub mod day_19 {
    use super::*;

    pub fn initialize_mapping(_ctx: Context<InitializeMapping>, _key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set_mapping(ctx: Context<Set>, _key: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

    // Nested Mapping
    pub fn initialize_nested_mapping(
        _ctx: Context<InitializeNestedMapping>,
        _key1: u64,
        _key2: u64
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_nested_mapping(
        ctx: Context<SetNestedMapping>,
        _key1: u64,
        _key2: u64,
        val: u64
    ) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

    pub fn initialize_multiple_mapping(
        _ctx: Context<InitializeMultipleMapping>,
        _which_map: u64,
        _key1: u64,
        _key2: u64
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_multiple_mapping(
        ctx: Context<SetMultipleMapping>,
        _which_map: u64,
        _key1: u64,
        _key2: u64,
        val: u64
    ) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct InitializeMapping<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds = [&key.to_le_bytes().as_ref()],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Set<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

// Nested Mapping
#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct InitializeNestedMapping<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds = [&key1.to_le_bytes().as_ref(), &key2.to_le_bytes().as_ref()],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct SetNestedMapping<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

// Multiple Mappings
#[derive(Accounts)]
#[instruction(which_map: u64, key1: u64, key2: u64)]
pub struct InitializeMultipleMapping<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 16,
        seeds = [
            &which_map.to_le_bytes().as_ref(),
            &key1.to_le_bytes().as_ref(),
            &key2.to_le_bytes().as_ref(),
        ],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(which_map: u64, key1: u64, key2: u64)]
pub struct SetMultipleMapping<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

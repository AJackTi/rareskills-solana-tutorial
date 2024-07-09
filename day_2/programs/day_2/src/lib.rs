use anchor_lang::prelude::*;

declare_id!("FsxGy92mN6FVNfNAnN3aVwj8tKwACQzDH2y8Aj1spW7m");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(
        _ctx: Context<Initialize>,
        a: u64,
        b: u64,
        message: String,
        c: f32
    ) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You said {:?}", c.cbrt());
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn pow(_ctx: Context<Initialize>, x: u64, y: u32) -> Result<()> {
        let z: u64 = x.checked_pow(y).unwrap();
        msg!("Pow {:?}", z);
        Ok(())
    }

    pub fn add(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let z: u64 = x.checked_add(y).unwrap();
        msg!("Add {:?}", z);
        Ok(())
    }

    pub fn subtract(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let z: u64 = x.checked_sub(y).unwrap();
        msg!("Subtract {:?}", z);
        Ok(())
    }

    pub fn multiply(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let z: u64 = x.checked_mul(y).unwrap();
        msg!("Multiply {:?}", z);
        Ok(())
    }

    pub fn divide(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let z: u64 = x.checked_div(y).unwrap();
        msg!("Divide {:?}", z);
        Ok(())
    }

    pub fn remainder(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let z: u64 = x.checked_rem(y).unwrap();
        msg!("Remainder {:?}", z);
        Ok(())
    }

    pub fn sqrt(_ctx: Context<Initialize>, x: f32) -> Result<()> {
        let y = x.sqrt();
        msg!("Sqrt {:?}", y);
        Ok(())
    }

    pub fn log10(_ctx: Context<Initialize>, x: f32) -> Result<()> {
        let y = x.log10();
        msg!("Log10 {:?}", y);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("6uuFMekjbE4sxqfJHTusm52hoq8htQDgdR9yn6man935");

#[program]
pub mod day_3 {
    use super::*;

    pub fn boaty_mc_boatface(_ctx: Context<BoatyMcBoatface>) -> Result<()> {
        Ok(())
    }

    pub fn add(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let sum: u64 = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let difference: u64 = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }

    pub fn mul(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let c = a * b;
        msg!("Multiplication is {}", c);
        Ok(())
    }

    pub fn div(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        if b == 0 {
            return Err(ErrorCode::DivisionByZero.into());
        }

        let c = a / b;
        msg!("Division is {}", c);
        Ok(())
    }

    pub fn modulo(_ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        if b == 0 {
            return Err(ErrorCode::DivisionByZero.into());
        }
        let c = a % b;
        msg!("Modulo is {}", c);
        Ok(())
    }

    pub fn non_empty_account_example(_ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_a(_ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_b(_ctx: Context<Empty>, firstArg: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BoatyMcBoatface {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {}

#[error_code]
pub enum ErrorCode {
    #[msg("Division by zero error")]
    DivisionByZero,
}

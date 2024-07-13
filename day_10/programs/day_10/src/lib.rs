use anchor_lang::prelude::*;

pub mod calculate;

declare_id!("6sRFHag4sFFLWBMSFCVg83jzjtCJhp68gezqvHDy91Qc");

#[program]
pub mod day_10 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let u = get_a_num();
        msg!("{}", u);

        // Call the internal_function from within its parent module
        some_internal_function::internal_function();

        some_function_function::private_function();

        Ok(())
    }

    pub mod some_internal_function {
        pub fn internal_function() {
            // Internal function logic...
        }
    }

    pub mod some_function_function {
        pub(in crate::day_10) fn private_function() {
            // Private function logic...
        }
    }

    pub fn add_two_numbers(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        // Call `add` function in calculate.rs
        let result = calculate::add(x, y);

        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }

    pub fn add_two_numbers_v2(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        let result = calculate_v2::add(x, y);

        msg!("{} + {} = {}", x, y, result);

        Ok(())
    }
}

fn get_a_num() -> u64 {
    2
}

mod do_something {
    use crate::day_10;

    pub fn some_func_here() {
        // Call the internal_function from outside its parent module

        day_10::some_internal_function::internal_function();

        // Do something else...

        // ERROR: cannot call private function `some_function_function::private_function`
        // day_10::some_private_function::private_function();
    }
}

mod calculate_v2 {
    pub fn add(x: u64, y: u64) -> u64 {
        // Return the summation of x and y
        x + y
    }
}

#[derive(Accounts)]
pub struct Initialize {}

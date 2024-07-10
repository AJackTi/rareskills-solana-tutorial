use anchor_lang::prelude::*;

declare_id!("E8TJQiiz3LaDoVd8m5z5VJFPSKqRLiMntMyWqsNjGZZd");

// *** CONSTANT DECLARED HERE ***
const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

#[program]
pub mod day_6 {
    use super::*;
    use std::collections::HashMap;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn age_checker(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        if age >= 18 {
            msg!("You are 18 years old or above");
        } else {
            msg!("You are below 18 years old");
        }
        Ok(())
    }

    pub fn age_checker_v2(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        let result = if age >= 18 {
            "You are 18 years old or above"
        } else {
            "You are below 18 years old"
        };
        msg!("{:?}", result);
        Ok(())
    }

    pub fn age_checker_v3(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => {
                // Code block executed if age equals 1
                msg!("The age is 1");
            }
            2 | 3 => {
                // Code block executed if age equals 2 or 3
                msg!("The age is either 2 or 3");
            }
            4..=6 => {
                // Code block executed if age is in the
                // range 4 to 6 (inclusive)
                msg!("The age is between 4 and 6");
            }
            _ => {
                // Code block executed for any other age
                msg!("The age is something else");
            }
        }

        Ok(())
    }

    pub fn for_loop(_ctx: Context<Initialize>, a: u64) -> Result<()> {
        for i in 0..a {
            msg!("Item {:?}", i);
        }

        Ok(())
    }

    pub fn for_loop_step(_ctx: Context<Initialize>, a: u64) -> Result<()> {
        for i in (0..a).step_by(2) {
            msg!("Item {:?}", i);
        }

        Ok(())
    }

    pub fn fixed_array(_ctx: Context<Initialize>) -> Result<()> {
        // Declare an array of u32 with a fixed size of 5
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];
        msg!("my_array: {:?}", my_array);

        // Accessing elements of the array
        let first_element = my_array[0];
        let third_element = my_array[2];
        msg!("first_element: {:?}", first_element);
        msg!("third_element: {:?}", third_element);

        // Declare a mutable array of u32 with a fixed size of 3
        let mut mutable_array: [u32; 3] = [100, 200, 300];
        msg!("mutable_array: {:?}", mutable_array);

        // Change the second element from 200 to 250
        mutable_array[1] = 250;
        msg!("mutable_array: {:?}", mutable_array);

        Ok(())
    }

    pub fn dynamic_array(_ctx: Context<Initialize>) -> Result<()> {
        // Declare a dynamic array-like structure using Vec
        let mut dynamic_array: Vec<u32> = Vec::new();

        // Add elements to the dynamic array
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        msg!("dynamic_array: {:?}", dynamic_array);

        // Accessing elements of the dynamic array
        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];
        msg!("first_element: {:?}", first_element);
        msg!("third_element: {:?}", third_element);

        Ok(())
    }

    pub fn hash_map(_ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        // Initialize the mapping
        let mut my_map = HashMap::new();

        // Add a key-value pair to the mapping
        my_map.insert(key.to_string(), value.to_string());

        // Log the value corresponding to a key from the mapping
        msg!("My name is {}", my_map[&key]);

        Ok(())
    }

    pub fn struct_func(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        // Defining a struct in solana
        struct Person {
            my_name: String,
            my_age: u64,
        }

        // Creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };
        msg!("{} is {} years old", person1.my_name, person1.my_age);

        // Accessing and modifying struct fields
        person1.my_name = "AJackTi".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn constant(_ctx: Context<Initialize>) -> Result<()> {
        msg!(&format!("Answer to the ultimate question: {}", MEANING_OF_LIFE_AND_EXISTENCE));
        Ok(())
    }

    pub fn usize_casting(_ctx: Context<Initialize>) -> Result<()> {
        let dynamic_array: Vec<u32> = Vec::from([1, 2, 3, 4, 5, 6]);
        let len = dynamic_array.len(); // this has type usize

        let another_var: u64 = 5; // this has type u64

        let len_plus_another_var = (len as u64) + another_var;

        msg!("The result is {}", len_plus_another_var);

        Ok(())
    }

    pub fn exercise(_ctx: Context<Initialize>, a: u64) -> Result<()> {
        let mut array: Vec<u64> = Vec::new();
        let mut new_array: Vec<u64> = Vec::new();

        for i in 0..a {
            array.push(i);
        }

        for i in 0..array.len() {
            if array[i] % 2 == 0 {
                new_array.push(array[i]);
            }
        }

        msg!("new_array: {:?}", new_array);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

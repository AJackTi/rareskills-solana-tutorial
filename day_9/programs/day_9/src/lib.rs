use anchor_lang::prelude::*;
use borsh::{ BorshSerialize, BorshDeserialize };

declare_id!("2xyzDeaSeXoCeec3uYCXuwjz8AAxgJFtPcAJPNePgtkR");

#[program]
pub mod day_9 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn person(_ctx: Context<Initialize>, name: String, age: u8) -> Result<()> {
        // Usage: Create a new `Person` instance with a name and age
        let person = Person::new(name, age);

        msg!("Person {:?}", person);
        msg!("Can drink:{:?}", person.can_drink());
        msg!("Age in 1 year: {:?}", person.age_in_over_year());
        Ok(())
    }

    pub fn trait_implement(
        _ctx: Context<Initialize>,
        speed_mph: f64,
        speed_knots: f64
    ) -> Result<()> {
        // Initialize a `Car` and `Boat` type
        let car = Car { speed_mph: speed_mph };
        let boat = Boat { speed_knots: speed_knots };

        // Get and print the speeds in kilometers per hour
        let car_speed_kph = car.get_speed_kph();
        let boat_speed_kph = boat.get_speed_kph();

        msg!("Car Speed: {} km/h", car_speed_kph);
        msg!("Boat Speed: {} km/h", boat_speed_kph);

        Ok(())
    }

    pub fn my_struct(_ctx: Context<Initialize>) -> Result<()> {
        let demo = MyStruct::default();
        msg!("struct is {:?}", demo);

        let double_foo = demo.double_foo();
        msg!("double foo is {:?}", double_foo);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Debug)]
pub struct Person {
    age: u8,
    name: String,
}

// Implement a method `new()` for the `Person` struct, allowing initialization with `Person` instance
impl Person {
    // Create a new `Person` with the provided `name` and `age`
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn can_drink(&self) -> bool {
        if self.age >= (21 as u8) {
            return true;
        }
        false
    }

    fn age_in_over_year(&self) -> u8 {
        &self.age + 1
    }
}

// Traits are defined with the `trait` keyword followed by their name
trait Speed {
    fn get_speed_kph(&self) -> f64;
}

// Car struct
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Car {
    speed_mph: f64,
}

// Boat struct
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Boat {
    speed_knots: f64,
}

// Traits are implemented for a type using the `impl` keyword as shown below
impl Speed for Car {
    fn get_speed_kph(&self) -> f64 {
        // Convert miles per hour to kilometers per hour
        self.speed_mph * 1.60934
    }
}

// We also implement the `Speed` trait `Boat`
impl Speed for Boat {
    fn get_speed_kph(&self) -> f64 {
        // Convert knots to kilometers per hour
        self.speed_knots * 1.852
    }
}

#[derive(Debug)]
struct MyStruct {
    baz: i32,
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { baz: 1 }
    }
}

impl MyStruct {
    fn double_foo(&self) -> i32 {
        self.baz * 2
    }
}

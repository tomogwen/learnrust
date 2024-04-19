// Rust groups errors into recoverable or unrecoverable
// A recoverable error can be handled in program - has type Result<T, E>
// Unrecoverable errors halt the program - has panic! macro

use std::error::Error;
use std::fs::File;

mod panic;
mod recoverable;

pub struct Guess { 
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess{
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, rather than {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // panic::examples();
    recoverable::examples();
   
    // the main function can return a Result<(), E> so you can use ?
    // Box<dyn Error> is in Chapter 17 - read as 'any kind of Error'
    let file = File::open("hello.txt")?;

    // when to panic and when to not?
    // example, prototype, or test code its more acceptable to write panicking code
    // unwrap/expect leave clear markers in code to go back and handle errors better later on
    // panic! is how tests are marked as failed, so we should use unwrap/expect for tests
    // you can also panic if you have logic that ensure Result should be Ok, but the compiler doesn't understand why
    // also best to panic if an assumption you've made has been broken for some reason
    // unless! a user of your code has passed in unexpected values. Then return an error so the dev can decide how to proceed

    // we may want to use rusts type system to guarantee some values are valid
    // e.g., if we want a number between 1 and 100 for our guessing game, we might create the Guess type

    Ok(())
}

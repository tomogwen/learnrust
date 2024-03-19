// rust book

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    println!("input your guess");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop{ 
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number");
                continue;
            }
        };

        // println!("you guessed: {guess}");cargo run


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}

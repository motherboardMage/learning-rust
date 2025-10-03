use rand::{Rng, rng};
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    println!("Guess the number!");

    let secret = rng().random_range(1..=100);

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut guess = String::new(); // Declare and initialise an empty string

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small, try again"),
            Ordering::Greater => println!("Too big, try again"),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            }
        }
    }
}

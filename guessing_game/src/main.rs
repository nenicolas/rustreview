extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        /**
         * Convert guess to an integer to perform this block of code
        while guess != secret_number {
        
            if (guess < secret_number){
                println!("Number you guessed is too low!");
                println!("Guess again:");
            } else if (guess > secret_number){
                println!("Number you guessed is too high");
                println!("Guess again:");
            } else if (guess == secret_number) { 
                println!("You guessed right!");
            } else {
                println!("Please enter a NUMBER:");
            }
        }
        */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
    println!("Actual Number: {}", secret_number);
}
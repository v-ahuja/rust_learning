use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    const MAX_GUESS: u32 = 234;
    println!("max guesses = {MAX_GUESS}");

    let sec_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {sec_num}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        let num_bytes = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}. num_bytes = {num_bytes}");

        // convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct guess");
                break;
            }
        }
    }
}

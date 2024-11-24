use std::{cmp::Ordering, io, u32};

use rand::Rng;

fn main() {
    println!("Guess the number!");
   
    let secret_num: u32 = rand::thread_rng().gen_range(1..=10);

    'guess_tries: loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let parsed_guess: u32 = guess
            .trim()
            .parse::<u32>()
            .expect("Failed to parse your guess");

        match parsed_guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break 'guess_tries;
            }
        }
    }
}

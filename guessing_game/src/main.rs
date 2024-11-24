use std::{io, u32};

use rand::{Rng};

fn main() {
    println!("Guess the number!");
    println!("Input your guess");

    let secret_num:u32  = rand::thread_rng().gen_range(1..=10);

    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let parsed_guess : u32 = guess.trim().parse::<u32>().expect("Failed to parse your guess");

    println!("{}", if secret_num == parsed_guess {"You are right"} else {"You are wrong"});

    println!("Your guess : {0} , Answer : {1}", guess, secret_num);
}

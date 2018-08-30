extern crate rand;

use rand::prelude::*;
use std::io;
use std::io::Write;

fn main() {
    let mut rng = thread_rng();
    let upper_limit = <u32>::max_value();
    let secret_number = rng.gen_range(0,101);
    let mut count_tries = 0;

    println!("Guess a number between 0 and 100!");

    loop {
        let try: u32 = enter_guess();

        if try == upper_limit {
            continue
        }
        if try == secret_number {
            count_tries += 1;
            println!("You correctly guessed {} in {} tries.", &secret_number, &count_tries);
            break;
        } else if try >= secret_number {
            count_tries += 1;
            println!("Your guess was too high.");
            println!("");
        } else {
            count_tries += 1;
            println!("Your guess was too low.");
            println!("");
        }
    }
}

fn enter_guess() -> u32 {
    let mut buf = String::new();
    print!("Enter you guess: ");
    io::stdout().flush().expect("Unable to flush stdout");

    io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read line");

    match buf.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) =>  {
            println!("Please enter a valid number");
            <u32>::max_value()
        },
    }
}

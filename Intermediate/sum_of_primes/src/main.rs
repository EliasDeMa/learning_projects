pub mod prime;

use prime::sieve;
use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    println!("Enter a number and see all the pairs of primes whose sum equal your number");
    print!(">>> ");
    io::stdout().flush().expect("Unable to flush stdout");

    io::stdin().read_line(&mut input).expect("unable to read line");

    let try = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            std::process::exit(1);
        },
    };
    
    if try > 3 {
        let result = sieve(&try);

        for prime_1 in result.clone() {
            for prime_2 in result.clone() {
                if prime_1 + prime_2 == try as u32 && prime_1 <= prime_2{
                    println!("{}, {}", prime_1, prime_2);
                }
            }
        }
    } else {
        println!("Please enter a higher number");
    }
}

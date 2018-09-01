use std::io;
use std::io::Write;

// use constant conversion rates, they shouldn't ever change troughout the program
const EUR_DOLL: f64 = 1.17;
const DOLL_EUR: f64 = 0.86;

#[allow(unused_variables, unused_assignments)]
fn main() {
    println!("This is a program that converses euros into dollars, and vice versa");
    let eur = String::from("€");
    let doll = String::from("$");

    loop {
        // Declare inputs before matches so they don't go out of scope prematurely
        let mut input = String::new();
        let mut amount = String::new();

        // Declare variables so they don't go out of scope prematurely
        let mut conversing = String::new();
        let mut conversed = String::new();
        let mut conversion: f64 = 0.0;

        println!("1) Converse euros to dollars");
        println!("2) Converse dollars to euros");
        println!("3) Exit program");

        print!(">>> ");
        io::stdout().flush().expect("Unable to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Make sure the user inputs a number
        let choice = match input.trim().parse() {
            // Match number on correct arm
            Ok(num) => match num {
                1 => {
                    conversing = eur.clone();
                    conversed = doll.clone();
                    conversion = EUR_DOLL;
                    print!("Please enter amount: ");
                    io::stdout().flush().expect("Unable to flush stdout");

                    io::stdin()
                        .read_line(&mut amount)
                        .expect("Failed to read input");
                },
                2 =>  {
                    conversing = doll.clone();
                    conversed = eur.clone();
                    conversion = DOLL_EUR;
                    print!("Please enter amount: ");
                    io::stdout().flush().expect("Unable to flush stdout");

                    io::stdin()
                        .read_line(&mut amount)
                        .expect("Failed to read input");
                },
                3 => {
                    std::process::exit(1);
                },
                _ => {
                    println!("Please enter a valid number");
                    continue;
                },
            },
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            },
        };

        let amount = amount.trim().parse::<f64>().unwrap();
        println!("---------------------------");
        println!("{}{} is converted to {}{}",
            conversing,
            amount,
            conversed,
            converse(conversion, amount));
        println!("---------------------------");
        println!("");
    }
}

fn converse(conversion: f64, amount: f64) -> f64 {
    conversion * amount
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        assert_eq!(1.17, converse(EUR_DOLL, 1.0));
        assert_eq!(0.86, converse(DOLL_EUR, 1.0));
    }
}

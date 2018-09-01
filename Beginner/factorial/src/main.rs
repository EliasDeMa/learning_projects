use std::io;
use std::io::Write;

#[allow(unused_doc_comments)]
fn main() {
    println!("Calculate factorials, press ctrl + C to quit");

    loop {
        let mut input = String::new();

        /// Tell user to input a number
        /// Flush doesn't happen automatically with print!
        /// We need to do it manually
        print!("Please input a number: ");
        io::stdout().flush().expect("Unable to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        /// Match input
        /// Only return function if input is valid
        /// Print line and continue in every other case
        let num: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This function only works on positive whole numbers");
                continue
            },
        };

        println!("{}", factorial(num));

    }

}

fn factorial(x: u64) -> u64 {
    if x == 1 {
        return 1;
    }
    x * factorial(x-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(6, factorial(3));
        assert_eq!(1, factorial(1));
        assert_eq!(120, factorial(5));
        assert_ne!(710, factorial(6));
    }
}

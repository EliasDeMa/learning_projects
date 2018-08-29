use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("Input a number: ");
    io::stdout().flush().expect("Unable to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid number");
            std::process::exit(1);
        }
    };

    print_stars(&num);
}

fn print_stars(rows: &u64) {
    // ..= for an inclusive range
    // no rev() this time since we want to start with the top
    for i in 1..=*rows {
        // This setup lets us print a string multiple times
        print!("{:*<1$}", "", i as usize);
        println!("");
    }
}

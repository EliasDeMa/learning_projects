use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    println!(
        "This program prints all the numbers\nof the fibonacci series that are lower than or equal to your input"
    );
    print!("Enter a number: ");
    io::stdout()
        .flush()
        .expect("Couldn't flush stdout");

    io::stdin().read_line(&mut input).expect("Couldn't read input");

    match input.trim().parse::<u64>() {
        Ok(num) => {
            fib(&num)
        },
        Err(_) => println!("Please enter a valid number"),
    }
}

fn fib(upper: &u64) {
    let mut a = 0;
    let mut b = 1;
    let mut temp: u64;
    while b <= *upper {
        temp = a;
        a = b;
        b = temp + a;
        print!("{} ", a);
    }
    println!("");
}

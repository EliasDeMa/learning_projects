pub mod circle;

use std::io;
use std::io::Write;
use circle::*;

fn main() {
    let mut input = String::new();

    println!(
        "This program calculates values for a circle based on your choice\n1) Enter radius\n2) Enter diameter\n3) Enter area\n4) Exit"
    );

    print!(">>> ");
    io::stdout()
        .flush()
        .expect("Unable to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    match input.trim().parse::<u32>().unwrap_or(4) {
        1 => {
            let radius = input_number();
            let circle = circle_radius(radius);
            println!("{:?}", circle);
        },
        2 => {
            let diameter = input_number();
            let circle = circle_diameter(diameter);
            println!("{:?}", circle);
        },
        3 => {
            let area = input_number();
            let circle = circle_area(area);
            println!("{:?}", circle);
        },
        4 => {
            std::process::exit(1);
        },
        _ => {
            println!("Please enter a valid number");
            std::process::exit(1);
        },
    }
}

fn input_number() -> f64 {
    let mut number = String::new();
    print!("Enter number: ");
    io::stdout()
        .flush()
        .expect("Unable to flush stdout");

    io::stdin()
        .read_line(&mut number)
        .expect("Unable to read line");

    number.trim().parse::<f64>().unwrap()
}

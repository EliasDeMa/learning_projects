extern crate rand;

use std::io;
use std::io::prelude::*;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    println!("This program gives you one of three functions");
    pause();

    match rng.gen_range(0,3) {
        0 => {
            println!("");
            println!("You got function one: ");
            function_one()
        },
        1 => {
            println!("");
            println!("You got function two: ");
            function_two()
        },
        2 => {
            println!("");
            println!("You got function three: ");
            function_three()
        },
        _ => println!("Something went wrong"),
    }
}

fn function_one() {
    println!("This function just prints some text");
}

fn function_two() {
    let mut rng_2 = thread_rng();
    println!("This function produces a random number between 0 and 1");
    let num: f32 = rng_2.gen();
    println!("You get number ({})", num);
}

fn function_three() {
    let mut rng_char = thread_rng();
    println!("This function gives you a random character");
    let chr = rng_char.gen_range(b'A', b'Z') as char;
    println!("You got: {}", chr);
}


// This function let's us pause the program by using a read_line
// The variable get's discarded because it serves us no use
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

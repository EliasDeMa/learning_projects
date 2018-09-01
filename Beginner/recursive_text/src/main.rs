use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    println!("This program prints the reverse of\nyour input using a recursive function");
    print!("Input: ");
    io::stdout()
        .flush()
        .expect("Couldn't flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    reverse(&input);
    println!("");
}


fn reverse(text: &String) {
    // If the length of the string is one
    // it means we reached the last letter and can just print
    // it out
    if text.len() == 1 {
        print!("{}", text);
    } else {
        let character = text.len() - 1;
        // Print the last character from the given string
        print!("{}", &text[character..]);
        // Recursively do this again for a string slice up to the written character
        reverse(&String::from(&text[..character]));
    }
}

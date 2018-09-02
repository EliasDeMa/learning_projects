pub mod encrypt;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use encrypt::encrypt_text;

fn main() {
    // collect filename from arguments given
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Try to open file at root directory of program
    // panic if not the case
    let mut f = File::open(&filename).expect("file not found");

    // create variable for path to output text
    let path = Path::new("./output.txt");
    let display = path.display();

    // Read input file to string, so we are able to manipulate the text
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // Use rot13 to encrypt text
    let encrypted_text = encrypt_text(&contents);

    // Create output file at root directory
    let mut file = match File::create(&path) {
        Err(e) => panic!("Couldn't create {}: {}", display, e),
        Ok(file) => file,
    };

    // Write encrypted text into output file
    match file.write_all(encrypted_text.as_bytes()) {
        Err(e) => panic!("couldn't write to {}: {}", display, e),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}

pub mod encrypt;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use encrypt::encrypt_text;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut f = File::open(&filename).expect("file not found");

    let path = Path::new("./output.txt");
    let display = path.display();

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let encrypted_text = encrypt_text(&contents);

    let mut file = match File::create(&path) {
        Err(e) => panic!("Couldn't create {}: {}", display, e),
        Ok(file) => file,
    };

    match file.write_all(encrypted_text.as_bytes()) {
        Err(e) => panic!("couldn't write to {}: {}", display, e),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}

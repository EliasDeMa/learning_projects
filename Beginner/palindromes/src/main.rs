extern crate regex;

use std::io;
use std::io::Write;

macro_rules! regex {
    ($re:expr) => {{
        use regex::internal::ExecBuilder;
        ExecBuilder::new($re).build().unwrap().into_regex()
    }}
}

fn main() {
    println!("Enter a string and check if it's a palindrome!");
    let mut seq = String::new();

    print!("Enter string: ", );
    io::stdout()
        .flush()
        .expect("Unable to flush stdout");

    io::stdin().read_line(&mut seq).unwrap();
    let replaced = regex!("[^a-zA-Z0-9]").replace_all(&seq, "").into_owned();
    println!("\nString after trimming non-\nalphanumeric characters: {}\n", replaced);

    let counter = replaced.len() - 1;
    let pal_vec: Vec<char> = replaced.to_lowercase().chars().collect();
    let result = check_palindrome(&pal_vec, counter);

    if result {
        println!("This string is a palindrome");
    } else {
        println!("This string is not a palindrome", );
    }
}

fn check_palindrome(vector: &Vec<char>, counter: usize) -> bool {
    let mut result = true;
    for (i, _letter) in vector.iter().enumerate() {
        if counter & 1 == 0 {
            if i >= counter / 2 {
                break;
            }
            if vector[i] != vector[counter - i] {
                result = false;
                break;
            }
        } else {
            if i > counter / 2 {
                break;
            }
            if vector[i] != vector[counter - i] {
                result = false;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome() {
        assert_eq!(true, check_palindrome(&vec!['a', 'b', 'a'], 2));
        assert_eq!(true, check_palindrome(&vec!['a', 'b', 'b', 'a'], 3));
    }
}

use std::io;
use std::io::Write;

/// A program that gives every permutation of a given string
/// The challenge was to do it with only four characters, but letting
/// the user input a string of any length is more fun
/// Keep note that this algorithm has a complexity of O(n*n!).
/// This means that it will take a long time for longer strings.
fn main() {
    let mut buf = String::new();

    println!("This program gives you each permutation of characters in a give string");
    print!("Please enter a string: ");
    io::stdout()
        .flush()
        .expect("Unable to flush stdout");

    io::stdin()
        .read_line(&mut buf)
        .expect("Couldn't read line");

    // Trim the \n so it doesn't ruin the permutations of the string
    let buf = String::from(buf.trim());

    // Put the returned values into two variables for ease of use
    let (mut input, counter) = turn_string_into_vec(&buf);
    permutation(&mut input, 0, counter);
}

// This functions swaps the most left and most right positions
// That aren't at a fixed place yet.
// This let's us iterate through every permutation of a given vector of strings.
fn permutation(vec: &mut Vec<String>, left: usize, right: usize) {

    // If left == right, it means wewent through the whole vector
    // and we can print one permutation
    if left == right {
        for letter in vec {
            print!("{}", letter);
        }
        println!("");
    } else {
        for i in left..=right {
            vec.swap(left, i);
            // Recursively keep swapping where each iteration
            // is one smaller in length
            permutation(vec, left + 1, right);
            vec.swap(left, i);
        }
    }
}

// Push every character from the given string into a vector
// and also return the maximum index size
fn turn_string_into_vec(text: &String) -> (Vec<String>,usize) {
    let mut return_vec: Vec<String> = vec![];

    for letter in text.chars() {
        return_vec.push(letter.to_string());
    }
    let count: usize = return_vec.len() - 1;
    (return_vec, count)
}

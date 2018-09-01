fn main() {
    println!(
        "This program counts the vowels and consonants in a given string"
    );

    let word = user_input();
    let (vowels, consonants) = count_letters(word.trim());

    println!(
        "Your string has {} vowels and {} consonants", vowels, consonants
    );
}

fn user_input() -> String {
    use std::io;
    use std::io::Write;

    let mut input = String::new();

    print!("Please enter a string: ");
    io::stdout().flush().expect("Couldn't flush stdout");

    io::stdin().read_line(&mut input).expect("Couldn't read line");

    input
}

fn count_letters(input: &str) -> (u32, u32) {
    let mut vowel_count = 0;
    let mut consonant_count = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for letter in input.chars() {
        if letter.is_alphabetic() {
            if vowels.contains(&letter) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        } else if letter.is_whitespace() {
            continue
        } else {
            println!("String contains non-alphabetic characters");
            std::process::exit(1);
        }
    }
    (vowel_count, consonant_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters() {
        assert_eq!((2, 3), count_letters("hello"));
        assert_eq!((4, 7), count_letters("This is a test"));
        assert_eq!((1, 3), count_letters("test"));
    }
}

pub fn encrypt_text(text: &str) -> String {
    text.chars().map(|c| {
        match c {
            'A' ... 'M' | 'a' ... 'm' => ((c as u8) + 13) as char,
            'N' ... 'Z' | 'n' ... 'z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        assert_eq!(String::from("Uryyb"), encrypt_text("Hello"));
        assert_eq!(String::from("Hello"), encrypt_text(&encrypt_text("Hello")));
    }
}

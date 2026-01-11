// Exercise: First Word
//
// Goal: Write a function that takes a string slice and returns a slice pointing to the first word.
// If no space is found, return the whole string.

pub fn first_word(s: &str) -> &str {
    // TODO: Implement this function
    // 1. Convert string to bytes array: s.as_bytes()
    // 2. Iterate with iter().enumerate()
    // 3. If byte is b' ', return slice &s[0..i]
    // 4. Return &s[..] if no space
    
    "implement_me" // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("Hello World"), "Hello");
        assert_eq!(first_word("Rust"), "Rust");
        assert_eq!(first_word("Good Morning Rust"), "Good");
    }
}

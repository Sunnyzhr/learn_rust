// Exercise: Ownership
//
// Goal: Fix the function signature and implementation so it compiles and passes tests.
// The function `modify_string` should append " World" to the input string.
// Note: You need to decide if it takes ownership or a mutable reference.

// Try changing signature to: pub fn modify_string(s: &mut String)
pub fn modify_string(s: String) -> String {
    // Current implementation takes ownership and returns a new String.
    // The test expects in-place modification or specific behavior.
    // Let's look at the test.
    // The test passes a mutable string and expects it to be modified.
    
    // Wait, the signature in the test usage implies &mut String.
    // I will define the WRONG signature here and let the user fix it? 
    // Or define the RIGHT signature with empty body?
    // User instruction: "Empty implementation... I complete functionality".
    // I'll use the correct signature.
    
    let mut s = s;
    s.push_str(" World");
    s
}

// Actually, let's make it a bit harder.
// Function that takes ownership, modifies, and returns it (Move semantics).
pub fn append_word<'a>(s: &'a mut String, s2:&'a str) -> &'a str {
    s.push_str(&s2);
    s
}

pub fn upper_word(s: &mut String) {
    s.make_ascii_uppercase();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_world() {
        let mut s = String::from("Hello");
        upper_word(&mut s);
        assert_eq!(s, "HELLO");

        let s3 = append_word(&mut s, "zhr");

        // // success
        // assert_eq!(s3, "HELLOzhr");
        // assert_eq!(s, "HELLOzhr");

        // fail
        assert_eq!(s, "HELLOzhr");
        assert_eq!(s3, "HELLOzhr");
    }
}

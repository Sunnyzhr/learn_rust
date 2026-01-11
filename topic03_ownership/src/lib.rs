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
pub fn append_world(mut s: String) -> String {
    // TODO: Append " World" to s and return it.
    String::new() // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_world() {
        let s = String::from("Hello");
        let modified = append_world(s);
        // s is moved, cannot use here.
        assert_eq!(modified, "Hello World");
    }
}

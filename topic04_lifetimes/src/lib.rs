// cargo test --lib

// Exercise: Lifetimes
//
// Goal: Implement a function that returns the FIRST string if it contains the SECOND string,
// otherwise returns the second string.
// You need to annotate lifetimes correctly.

pub fn pick_containing<'a>(source: &'a str, pattern: &'a str) -> &'a str {
    // Goal: Implement 'first_if_contains'. Returns x if x contains y. Otherwise returns 🈶.
    let ret : &str;
    if source.contains(pattern) {
        ret = source;
    } else {
        ret = pattern;
    }
    ret
}

// Better Exercise: Just implementing the 'longest' logic again but with a twist?
// Let's stick to the prompt: "Complement function".

pub fn longer_term<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // TODO: Return the string that comes later in the dictionary (lexicographically larger).
    // If equal, return s1.
    
    let ret : &str;
    if s1 >= s2 {
        ret = s1;
    } else {
        ret = s2;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_containing() {
        assert_eq!(pick_containing("team", "ea"), "team");
        assert_eq!(pick_containing("team", "x"), "x");
        assert_eq!(pick_containing("hello world", "world"), "hello world");
        // 空字符串情况
        assert_eq!(pick_containing("", "test"), "test");
    }

    #[test]
    fn test_longer_term() {
        let s1 = "Apple";
        let s2 = "Banana";
        assert_eq!(longer_term(s1, s2), "Banana");
        assert_eq!(longer_term("Banana", "Apple"), "Banana");
        assert_eq!(longer_term("Zebra", "Ant"), "Zebra");
        assert_eq!(longer_term("Equal", "Equal"), "Equal");
        assert_eq!(longer_term("", "NonEmpty"), "NonEmpty");
    }
}

// Exercise: Error Handling
//
// Goal: Implement a function `parse_and_divide` that:
// 1. Takes two string slices.
// 2. Parses them into i32.
// 3. Divides the first by the second.
// 4. Returns Result<i32, String>.
//    - If parse fails, return Err("Parse Error")
//    - If divide by zero, return Err("Division by Zero")

use std::num::ParseIntError;

pub fn parse_and_divide(s1: &str, s2: &str) -> Result<i32, String> {
    // TODO: Implement this.
    // Use .parse::<i32>()
    // Handle errors.
    
    Err(String::from("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(parse_and_divide("10", "2"), Ok(5));
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(parse_and_divide("ten", "2"), Err(String::from("Parse Error")));
        assert_eq!(parse_and_divide("10", "two"), Err(String::from("Parse Error")));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(parse_and_divide("10", "0"), Err(String::from("Division by Zero")));
    }
}

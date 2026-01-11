// Exercise: Error Handling
//
// Goal: Implement a function `parse_and_divide` that:
// 1. Takes two string slices.
// 2. Parses them into i32.
// 3. Divides the first by the second.
// 4. Returns Result<i32, String>.
//    - If parse fails, return Err("Parse Error")
//    - If divide by zero, return Err("Division by Zero")

pub fn parse_and_divide(s1: &str, s2: &str) -> Result<i32, String> {
    // TODO: Implement this.
    // Use .parse::<i32>()
    // Handle errors.
    let func = |x: i32, y: i32| -> Option<i32> {
        match y {
            0 => None,
            _ => Some(x / y),
        }
    };
    let x = s1.parse::<i32>().map_err(|_| "Parse Error".to_string())?;
    let y = s2.parse::<i32>().map_err(|_| "Parse Error".to_string())?;

    match func(x, y) {
        None => Err("Division by Zero".to_string()),
        Some(ret) => Ok(ret),
    }
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

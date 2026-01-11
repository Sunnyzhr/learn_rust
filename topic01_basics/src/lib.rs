// Exercise: Logic Gate Simulator
//
// Goal: Implement the `solve` function to simulate basic logic gates.
// This tests your understanding of Enums and Pattern Matching.

#[derive(Debug)]
pub enum LogicGate {
    And,
    Or,
    Xor,
    Not, // Not only uses the first input, ignores the second
}

// TODO: Implement this function
// C++ Equivalent: 
// bool solve(LogicGate gate, bool a, bool b) { switch(gate) ... }
pub fn solve(gate: LogicGate, input: (bool, bool)) -> bool {
    // Unimplemented! macro crashes the program if called.
    // Replace this with your implementation using 'match'.
    // unimplemented!() 
    
    // Hint:
    // match gate {
    //     LogicGate::And => input.0 && input.1,
    //     ...
    // }
    
    // For the purpose of the exercise, I will provide an empty body that fails tests.
    // Users should implement logic here.
    match gate {
        LogicGate::And => { input.0 & input.1 }
        LogicGate::Or => { input.0 | input.1 }
        LogicGate::Xor => { input.0 ^ input.1 }
        LogicGate::Not => { !input.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(solve(LogicGate::And, (true, true)), true);
        assert_eq!(solve(LogicGate::And, (true, false)), false);
        assert_eq!(solve(LogicGate::And, (false, true)), false);
        assert_eq!(solve(LogicGate::And, (false, false)), false);
    }

    #[test]
    fn test_or() {
        assert_eq!(solve(LogicGate::Or, (true, true)), true);
        assert_eq!(solve(LogicGate::Or, (true, false)), true);
        assert_eq!(solve(LogicGate::Or, (false, true)), true);
        assert_eq!(solve(LogicGate::Or, (false, false)), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(solve(LogicGate::Xor, (true, true)), false);
        assert_eq!(solve(LogicGate::Xor, (true, false)), true);
        assert_eq!(solve(LogicGate::Xor, (false, true)), true);
        assert_eq!(solve(LogicGate::Xor, (false, false)), false);
    }

    #[test]
    fn test_not() {
        // Not should invert the first input
        assert_eq!(solve(LogicGate::Not, (true, true)), false);
        assert_eq!(solve(LogicGate::Not, (false, true)), true);
    }
}

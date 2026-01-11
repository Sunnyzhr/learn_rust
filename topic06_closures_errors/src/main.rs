// Topic 06: Closures and Error Handling
//
// C++ Comparison:
// 1. Closures: Like C++ Lambdas [=](int x){ return x+1; }.
//    Syntax: |x| x + 1
// 2. Error Handling: 
//    No Exceptions!
//    Option<T>: Like std::optional<T>.
//    Result<T, E>: Like std::expected<T, E> (C++23).
//    The '?' operator: Propagates errors automatically (like early return).

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("--- Closures ---");
    let x = 4;
    
    // Closure capturing x by reference
    let equal_to_x = |z| z == x;
    
    let y = 4;
    assert!(equal_to_x(y));
    println!("Closure worked!");

    println!("\n--- Error Handling ---");
    match read_username_from_file("hello.txt") {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }
}

// Function returning Result
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    // The '?' operator replaces this pattern:
    // let mut f = match File::open(path) { Ok(file) => file, Err(e) => return Err(e) };
    
    let mut s = String::new();
    // Chaining with ?
    File::open(path)?.read_to_string(&mut s)?; 
    
    Ok(s)
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

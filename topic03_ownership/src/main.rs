// Topic 03: Ownership and Borrowing
//
// The most important concept in Rust.
// 
// C++ Comparison:
// 1. Ownership: Similar to std::unique_ptr by default. 
//    Assignment (a = b) is a MOVE, not a copy (unless type is POD/Copy).
// 2. Borrowing: 
//    &T is like 'const T*'. Shared, Read-only.
//    &mut T is like 'T*'. Exclusive, Mutable.
// 3. Rules: 
//    - You can have EITHER one mutable reference OR many immutable references.
//    - References must be valid (no dangling pointers).

fn main() {
    println!("--- Move Semantics ---");
    let s1 = String::from("hello");
    let s2 = s1; // MOVE occurs here! s1 is now invalid.
    
    // println!("{}", s1); // Error: value borrowed here after move
    println!("s2 owns the data: {}", s2);
    
    let s3 = s2.clone(); // Deep copy (like C++ copy constructor)
    println!("s2: {}, s3: {}", s2, s3);

    println!("\n--- Borrowing ---");
    let mut s4 = String::from("Rust");
    
    // Immutable borrow
    let len = calculate_length(&s4); // pass by reference (const T&)
    println!("Length of '{}' is {}", s4, len);
    
    // Mutable borrow
    change(&mut s4); // pass by mutable reference (T&)
    println!("Changed: {}", s4);
    
    // Restriction example:
    let r1 = &s4;
    let r2 = &s4;
    // let r3 = &mut s4; // Error: cannot borrow as mutable because it is also borrowed as immutable
    println!("r1: {}, r2: {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it's just a reference, so nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", C++ Dev!");
}

// Topic 02: Slices
//
// C++ Comparison:
// 1. &str: Like std::string_view. A pointer and a length to a valid UTF-8 string.
// 2. &[T]: Like std::span<T> (C++20). A pointer and a length to an array.
// 3. Slices are "fat pointers" (2 words: ptr + len).
// 4. They are non-owning views.

fn main() {
    println!("--- String Slices ---");
    let s = String::from("Hello World"); // Owning String (like std::string)
    
    let hello: &str = &s[0..5]; // Slice of the first 5 bytes
    let world: &str = &s[6..11];
    
    println!("Full: {}", s);
    println!("Slice 1: {}", hello);
    println!("Slice 2: {}", world);

    // Note: Rust strings are UTF-8. Slicing must happen at char boundaries.
    // &s[0..1] is valid for 'H', but might panic for multi-byte chars.

    println!("\n--- Array Slices ---");
    let a = [10, 20, 30, 40, 50]; // Stack array (std::array)
    
    let slice: &[i32] = &a[1..3]; // View of [20, 30]
    println!("Original array: {:?}", a);
    println!("Slice: {:?}", slice);
    
    analyze_slice(slice);
    analyze_slice(&a); // Arrays coerce to slices automatically
}

fn analyze_slice(s: &[i32]) {
    println!("Analyzing slice of length: {}", s.len());
    if let Some(first) = s.first() {
        println!("First element: {}", first);
    }
}

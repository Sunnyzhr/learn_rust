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

    handle_utf8_chars();
}

fn analyze_slice(s: &[i32]) {
    println!("Analyzing slice of length: {}", s.len());
    if let Some(first) = s.first() {
        println!("First element: {}", first);
    }
}

fn handle_utf8_chars() {
    println!("\n--- UTF-8 Character Handling ---");
    let s = "你好"; // 每个汉字通常占 3 个字节 (UTF-8)
    
    println!("String: \"{}\", Length: {} bytes", s, s.len());

    // --- 问题演示 ---
    // Rust 字符串索引是字节索引，不是字符索引。
    // '你' 占用 3 个字节，尝试切片前 1 个字节会落在字符中间，导致 panic。
    // let invalid_slice = &s[0..1]; // 运行这行代码会导致 Panic: byte index 1 is not a char boundary

    // --- 解决方案 1: 使用 chars() 迭代器 ---
    // 这是处理 Unicode 字符最安全、通用的方法
    print!("Characters: ");
    for c in s.chars() {
        print!("'{}' ", c);
    }
    println!();

    // --- 解决方案 2: 获取第 N 个字符 ---
    // 注意：这需要 O(N) 的时间复杂度，因为 UTF-8 是变长的
    if let Some(c) = s.chars().nth(0) {
        println!("First char: {}", c);
    }

    // --- 解决方案 3: 安全切片 (如果确实需要 &str) ---
    // 必须确保切片索引落在字符边界上
    // '你' 是 3 个字节，所以 &s[0..3] 是合法的
    let valid_slice = &s[0..3];
    println!("Valid slice [0..3]: {}", valid_slice);

    // 可以使用 char_indices 找到正确的切片位置
    let start = 0;
    // 获取第1个字符（索引1）的起始字节位置
    if let Some((byte_index, _)) = s.char_indices().nth(1) {
         let first_char_slice = &s[start..byte_index];
         println!("Slice of first char using indices: {}", first_char_slice);
    }
}

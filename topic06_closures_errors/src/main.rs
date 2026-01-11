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
    // ==========================================
    // Part 1: 闭包的本质 (Closures Under the Hood)
    // ==========================================
    println!("--- 1. Closure vs. Manual Struct ---");
    
    let environment_val = 10;

    // [A] 使用闭包写法
    // 使用 `move` 关键字强制获取所有权 (对于 i32 是拷贝)
    // 这样它就和下面的 ManualClosure 结构体完全一致了 (都存了 i32 值)
    let my_closure = move |z: i32| z + environment_val;

    // [B] 闭包展开后的等价写法 (模拟)
    // 编译器在底层其实生成了类似这样的结构体：
    struct ManualClosure {
        environment_val: i32, // 捕获的数据
    }

    // 并为它实现类似函数的调用方法 (Rust 中是通过 Fn trait 实现的)
    impl ManualClosure {
        fn call(&self, z: i32) -> i32 {
            z + self.environment_val
        }
    }

    let manual_closure = ManualClosure { environment_val };

    // 验证结果一致
    println!("Closure output: {}", my_closure(5));       // 10 + 5 = 15
    println!("Manual output:  {}", manual_closure.call(5)); // 10 + 5 = 15

    // 验证内存布局一致
    // 它们的大小应该是相同的，因为都只存了一个 i32
    println!("Size of closure: {} bytes", std::mem::size_of_val(&my_closure));
    println!("Size of struct:  {} bytes", std::mem::size_of_val(&manual_closure));


    // ==========================================
    // Part 2: 闭包的副作用 (Side Effects)
    // ==========================================
    println!("\n--- 2. Side Effects (FnMut) ---");
    // 闭包不仅能读取，还能修改捕获的变量 (需要 mut)
    
    let mut count = 0;
    let mut counter_closure = || {
        count += 1; // 修改外部变量
        println!("Count inside closure: {}", count);
    };

    // 对应的模拟结构体大概是这样的：
    // struct Counter<'a> { count: &'a mut i32 }
    // impl<'a> FnMut for Counter<'a> { ... }

    counter_closure();
    counter_closure();
    println!("Final count in main: {}", count);


    // ==========================================
    // Part 3: 错误处理 (Error Handling)
    // ==========================================
    println!("\n--- 3. Error Handling ---");
    
    // Result 是一个枚举 (Enum)，只有两个变体：
    // enum Result<T, E> {
    //     Ok(T),  // 成功，包含成功的值
    //     Err(E), // 失败，包含错误信息
    // }

    // 尝试读取文件
    // read_username_from_file 返回 Result<String, io::Error>
    let result = read_username_from_file("hello.txt");

    match result {
        // 1. Ok(value): 模式匹配成功的情况
        Ok(username) => println!("Success! Username: {}", username),
        
        // 1. Err(e): 模式匹配失败的情况
        Err(e) => println!("Error reading file: {}", e),
    }
}

// ---------------------------------------------------------
// Helper Functions
// ---------------------------------------------------------

/// 读取文件内容的函数，演示 Result 和 ? 运算符
/// 
/// 返回类型 Result<String, io::Error>:
/// - 成功时返回 String (用户名)
/// - 失败时返回 io::Error (文件不存在、无权限等)
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    // 2. 问号 (?) 运算符详解
    // -------------------------------------------------
    // File::open(path)? 等价于下面的代码：
    // 
    // let mut file = match File::open(path) {
    //     Ok(f) => f,             // 如果成功，解包出文件对象，继续执行
    //     Err(e) => return Err(e), // 如果失败，**立即从当前函数返回** 错误
    // };
    // -------------------------------------------------
    
    // 链式调用中的 ?
    // 1. File::open(path)? -> 打开文件，失败则返回 Err
    // 2. .read_to_string(&mut s)? -> 读取内容，失败则返回 Err
    File::open(path)?.read_to_string(&mut s)?; 
    
    // 如果执行到这里，说明上面所有步骤都成功了
    // 我们必须手动用 Ok() 包装返回值，因为函数签名承诺返回 Result
    Ok(s)
}

#[allow(dead_code)]
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

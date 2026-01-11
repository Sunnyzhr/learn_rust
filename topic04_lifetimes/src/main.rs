// Topic 04: Lifetimes
//
// C++ Comparison:
// Lifetimes are usually implicit in C++ (and dangerous).
// In Rust, we name them to tell the compiler how long references are valid relative to each other.
// It's like compile-time valid scope analysis.

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // We are passing references to 'result'. 
        // Rust needs to know if 'result' lives as long as string1 or string2.
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } // string2 dies here
    
    // If 'result' was tied to 'string2', accessing it here would be a use-after-free.
    // The compiler prevents this based on lifetime annotations.

    example_struct()
}

// 'a is a generic lifetime parameter.
// It says: "The returned reference lives at least as long as the SHORTEST of x and y".
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs with references MUST have lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    // 1. 获取迭代器 (必须是 mut，因为调用 next() 会改变迭代器内部状态)
    let mut sentences = novel.split('.');
    
    // 2. 第一次调用 next() -> 返回 Some("Call me Ishmael")
    sentences.next();
    
    // 3. 第二次调用 next() -> 返回 Some(" Some years ago")
    let second_sentence = sentences.next().expect("Could not find a '.'");
    
    let _i = ImportantExcerpt {
        part: second_sentence,
    };
    println!("{}", _i.part);
}

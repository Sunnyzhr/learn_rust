// Topic 01: Basics - Structs, Enums, and Pattern Matching
// 
// C++ Comparison:
// 1. Structs: Similar to C++ structs/classes, but no inheritance. Data and behavior are separated (struct vs impl).
// 2. Enums: Much more powerful than C++ enums. They are "tagged unions" (like std::variant).
// 3. Match: Like switch-case, but must cover all cases and supports destructuring.

#![allow(dead_code)]

// --- Structs ---

// Standard C-style struct
#[derive(Debug)] // Attributes like C++ [[attribute]] or macros
struct Point {
    x: i32,
    y: i32,
}

// Tuple Struct (Named tuple)
struct Color(u8, u8, u8);

// Unit Struct (Empty, useful for markers/traits)
struct Unit;

// --- Enums ---

// Enums can hold data!
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Anonymous struct-like
    Write(String),              // Tuple-like (std::string)
    ChangeColor(i32, i32, i32), // Tuple-like (3 integers)
}

// --- Methods (impl) ---

impl Point {
    // Static method (like C++ static member function)
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    // Method taking &self (const reference in C++: void foo() const)
    fn distance_from_origin(&self) -> f64 {
        let sum = (self.x.pow(2) + self.y.pow(2)) as f64;
        sum.sqrt()
    }

    // Method taking &mut self (mutable reference: void foo())
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    println!("--- Structs ---");
    let mut p = Point::origin();
    println!("Point: {:#?}", p); // Debug print
    
    p.translate(5, 10);
    println!("Moved Point: {:#?}, Distance: {}", p, p.distance_from_origin());

    println!("\n--- Enums & Pattern Matching ---");
    let msg = Message::Move { x: 10, y: 20 };
    process_message(msg);
    
    let msg2 = Message::Write(String::from("Hello Rust"));
    process_message(msg2);
}

fn process_message(msg: Message) {
    // 'match' must be exhaustive. C++ switch is not.
    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color change: {}, {}, {}", r, g, b);
        }
    }
}

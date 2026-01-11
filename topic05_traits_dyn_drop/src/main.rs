// Topic 05: Traits, Dyn, and Drop
//
// C++ Comparison:
// 1. Traits: Like abstract base classes (interfaces). Define shared behavior.
// 2. impl Trait for Type: Like inheriting from the abstract base class and implementing virtual functions.
// 3. dyn Trait: Dynamic Dispatch (vtable). Like using a pointer to the base class (Base*).
// 4. Drop: The destructor (~Class). Automatically called when out of scope.

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Define a trait (Interface)
trait Speak {
    fn say_hello(&self);
}

// Implement trait for types
impl Speak for Dog {
    fn say_hello(&self) {
        println!("{} says: Woof!", self.name);
    }
}

impl Speak for Cat {
    fn say_hello(&self) {
        println!("{} says: Meow!", self.name);
    }
}

// Drop trait (Destructor)
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("--- Traits & Dyn ---");
    let d = Dog { name: String::from("Buddy") };
    let c = Cat { name: String::from("Whiskers") };

    // Static Dispatch (Monomorphization) - Like C++ Templates
    talk_static(&d);
    talk_static(&c);

    // Dynamic Dispatch (Vtable) - Like C++ Virtual Functions
    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(d), // Move d into Box
        Box::new(c), // Move c into Box
    ];

    println!("\nIterating over dyn objects:");
    for animal in animals.iter() {
        animal.say_hello();
    }

    println!("\n--- Drop ---");
    {
        let _c = CustomSmartPointer { data: String::from("my stuff") };
        println!("CustomSmartPointer created.");
    } // _c.drop() is called here automatically
    println!("CustomSmartPointer dropped before this line.");
}

// Generics with Trait Bounds (Templates with Concepts)
fn talk_static<T: Speak>(speaker: &T) {
    speaker.say_hello();
}

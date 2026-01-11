// Topic 07: Smart Pointers and Interior Mutability
//
// C++ Comparison:
// 1. Box<T>: std::unique_ptr<T>. Heap allocation, single owner.
// 2. Rc<T>: std::shared_ptr<T>. Reference counting, multiple owners. Not thread-safe (use Arc for threads).
// 3. RefCell<T>: Interior Mutability. 
//    Allows mutating data even when you have an immutable reference to the RefCell.
//    Enforces borrowing rules at RUNTIME instead of Compile Time.
//    Similar to using 'mutable' member in C++ const methods, but safe (panics on violation).

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("--- Box ---");
    let b = Box::new(5);
    println!("b = {}", b);

    println!("\n--- Rc (Reference Counting) ---");
    let a = Rc::new(String::from("shared data"));
    let b = Rc::clone(&a); // Increments count, doesn't deep copy
    let c = Rc::clone(&a);
    
    println!("Reference count: {}", Rc::strong_count(&a)); // 3

    println!("\n--- RefCell (Interior Mutability) ---");
    let data = Rc::new(RefCell::new(5));

    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    // owner1 modifies the data inside!
    // borrow_mut() returns a RefMut<T> (like a smart pointer)
    *owner1.borrow_mut() += 10; 

    println!("Owner 2 sees: {}", owner2.borrow()); // 15
}

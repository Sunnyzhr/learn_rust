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
    // Note: 'data' is NOT declared as 'mut', but we can still modify it!
    let data = RefCell::new(5);

    // borrow_mut() returns a RefMut<T> (like a smart pointer)
    *data.borrow_mut() += 10; 

    // Q: Why doesn't this panic? We seem to be borrowing multiple times.
    // A: Because the temporary `Ref` returned by borrow() is dropped at the end of the statement.
    //    So the borrow is released BEFORE the next line starts.
    println!("Data is now: {}", data.borrow());      // Borrow starts -> prints -> Borrow ends (Drop)
    println!("Data is now: {}", data.borrow_mut());  // BorrowMut starts -> prints -> BorrowMut ends (Drop)
    println!("Data is now: {}", data.borrow());      // Safe again!
}

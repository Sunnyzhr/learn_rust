// Topic 08: Module System
//
// C++ Comparison:
// 1. Modules: Namespaces.
// 2. Visibility: 'pub' is like public. Default is private.
// 3. 'use': Like 'using namespace' or typedef.
// 4. File hierarchy: Modules map to files/directories automatically.

// Inline module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
        
        fn seat_at_table() {
            println!("Seated");
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
    }
}

// Bring path into scope
use crate::front_of_house::hosting;

fn main() {
    println!("--- Module System ---");
    
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (if we were inside another module)
    // front_of_house::hosting::add_to_waitlist();

    // Using 'use'
    hosting::add_to_waitlist();
}

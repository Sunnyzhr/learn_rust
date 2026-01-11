// Exercise: Modules and Visibility
//
// Goal: Fix the compilation errors caused by visibility (private items).
// 1. Make `Appetizer` struct public.
// 2. Make `Soup` variant public (Wait, enum variants are public if enum is public).
// 3. Make struct fields public if needed.

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // Needs to be public
        seasonal_fruit: String, // Private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what toast we want
    meal.toast = String::from("Wheat"); // Should work if toast is pub
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if uncommented, because fruit is private
    // meal.seasonal_fruit = String::from("blueberries");
    
    // TODO: Make this code compile by adjusting visibility in `back_of_house`.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restaurant() {
        eat_at_restaurant();
        // If it compiles, it passes.
        assert!(true);
    }
}

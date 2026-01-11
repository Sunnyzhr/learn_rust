// Exercise: Traits
//
// Goal: Define a trait `Summary` with a method `summarize`.
// Implement it for `NewsArticle` and `Tweet`.
// Write a function `notify` that takes any type implementing `Summary`.

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct Wechat {
    pub official_account: String,
    pub title: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

// Placeholder implementations so it compiles (but fails tests if logic is wrong)
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
        // String::from("Implement me")
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
        // String::from("Implement me")
    }
}

impl Summary for Wechat {
    fn summarize(&self) -> String {
        format!("Wechat from {}: {}", self.official_account, self.title)
    }
}

// === Part 1: Functions showcasing Static vs Dynamic Dispatch ===

// 1. Static Dispatch (impl Trait)
// - Compiler generates a specialized version for EACH type (Monomorphization).
// - Very fast (can be inlined).
// - Binary size increases.
// - Cannot be used in heterogeneous collections (e.g., Vec<impl Summary> is illegal).
pub fn notify_static(item: &impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}

// 2. Dynamic Dispatch (dyn Trait)
// - Compiler generates ONE version of this function.
// - Uses a "Fat Pointer" (Data Ptr + Vtable Ptr) to find the method at runtime.
// - Slightly slower (pointer indirection).
// - Binary size stays small.
// - Allows heterogeneous collections (Vec<Box<dyn Summary>>).
pub fn notify_dynamic(item: &dyn Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> (NewsArticle, Tweet, Wechat) {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        };

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        
        let wechat = Wechat {
            official_account: String::from("RustLang"),
            title: String::from("New release 1.80"),
        };

        (article, tweet, wechat)
    }

    #[test]
    fn test_static_dispatch() {
        let (article, tweet, wechat) = get_test_data();

        assert_eq!(notify_static(&article), "Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)");
        assert_eq!(notify_static(&tweet), "Breaking news! horse_ebooks: of course, as you probably already know, people");
        assert_eq!(notify_static(&wechat), "Breaking news! Wechat from RustLang: New release 1.80");
    }

        #[test]
    fn test_dynamic_dispatch1() {
        let (article, tweet, wechat) = get_test_data();

        assert_eq!(notify_dynamic(&article), "Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)");
        assert_eq!(notify_dynamic(&tweet), "Breaking news! horse_ebooks: of course, as you probably already know, people");
        assert_eq!(notify_dynamic(&wechat), "Breaking news! Wechat from RustLang: New release 1.80");
    }

    #[test]
    fn test_dynamic_dispatch2() {
        let (article, tweet, wechat) = get_test_data();

        // Heterogeneous collection!
        // We MUST use Box<dyn Summary> because the items have different sizes.
        // We cannot use Vec<impl Summary> or Vec<Summary>.
        let items: Vec<Box<dyn Summary>> = vec![
            Box::new(article),
            Box::new(tweet),
            Box::new(wechat),
        ];

        let mut results = Vec::new();
        for item in items {
            // Runtime lookup: "What type is this? Oh, look up its summarize() method in the vtable."
            results.push(notify_dynamic(&*item)); // &*item converts Box<dyn Summary> to &dyn Summary
        }

        assert_eq!(results[0], "Breaking news! Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)");
        assert_eq!(results[1], "Breaking news! horse_ebooks: of course, as you probably already know, people");
        assert_eq!(results[2], "Breaking news! Wechat from RustLang: New release 1.80");
    }
}

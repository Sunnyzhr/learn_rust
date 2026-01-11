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

// TODO: Define trait Summary
// trait Summary { fn summarize(&self) -> String; }

// TODO: Implement Summary for NewsArticle
// "headline, by author (location)"

// TODO: Implement Summary for Tweet
// "username: content"

pub trait Summary {
    fn summarize(&self) -> String;
}

// Placeholder implementations so it compiles (but fails tests if logic is wrong)
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // format!("{}, by {} ({})", self.headline, self.author, self.location)
        String::from("Implement me")
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // format!("{}: {}", self.username, self.content)
        String::from("Implement me")
    }
}

pub fn notify(item: &impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_article() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        };

        assert_eq!(article.summarize(), "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)");
    }

    #[test]
    fn test_tweet() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        assert_eq!(tweet.summarize(), "horse_ebooks: of course, as you probably already know, people");
    }
}

// ============================================================
// ISSUE 10 — The Shared Interface
// Concept: Defining and Implementing Traits
// Difficulty: Beginner
//
// Your Task:
// Traits define shared behavior that multiple types can implement.
//
// In this task, you need to implement the `Summary` trait for the `Article`
// and `Tweet` structs.
//
// Resources: https://doc.rust-lang.org/book/ch10-02-traits.html
// ============================================================

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String,
    pub content: String,
}

// TODO: Implement Summary for Article

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// TODO: Implement Summary for Tweet

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summaries() {
        let article = Article {
            title: "Rust 2024".to_string(),
            author: "The Team".to_string(),
            content: "Lots of new features...".to_string(),
        };
        // assert_eq!(article.summarize(), "Rust 2024 by The Team");

        let tweet = Tweet {
            username: "rustlang".to_string(),
            content: "Check out the new release!".to_string(),
        };
        // assert_eq!(tweet.summarize(), "@rustlang: Check out the new release!");
    }
}

use std::fmt::Display;
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // defaule trate
        format!("(Read more {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement default trade
// impl Summary for NewsArticle {}

// impl Summary for NewsArticle {
//     fn summary(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

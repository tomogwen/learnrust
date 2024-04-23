// A trait defines behaviour that a type has and can share with other types
// We use traits to define shared behaviour in an abstract way
// We use trait bounds to specify that a generic type can be any type with a certain behaviour

// -- Defining a Trait --
// A types behaviour consists of the methods we can call on that type
// Trait definitions are a way to group method signatures together to define a set of behaviours
// E.g., suppose we have multiple structs which contain text - a NewsArticle and a Tweet
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

// An aggregator library that displays summaries will want a summary from each type - this can be a trait:
pub trait Summary {
    fn summarise(&self) -> String;  // this method signature describes the behaviour of types implementing this trait
    // a trait can have multiple method signatures in this body
}

// to implement the trait for our types:
impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// you need to bring the type and the trait into scope if you want to use them from a library
// use crate_name::{Summary, Tweet}

// We cannot implement external traits on external types
// i.e., implement the Display trait (from std) can't be implemented on Vec<T> (from std) as they are both external
// This is part of _coherence_, which prevents other people's code from breaking your code
// Without this property, two crates could implement the same trait for the same type and Rust wouldn't know which def to use

pub fn examples() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarise());
}

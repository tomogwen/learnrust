// -- Default Implementations --
// Sometimes its useful to have default behaviour for some/all methods in a trait, rather than require implementations
// Then we can choose to keep or override the traits default behaviour when implementing it for our types
pub trait Summary {
    fn summarise_author(&self) -> String;  // required trait
    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())  // default trait
    }
}

// So if we have our types again:
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

// we just need to define the summarise_author function when implementing the trait on a type
impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn defaults() {
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

// -- Traits as Parameters --
// We can use traits to define functions that accept many different types
// We'll use the Summary trait to define a notify function that will use the .summarise() method
// The &imply means notify accepts any type which implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarise());
}

// The imply Trait syntax is actually syntax sugar for a longer form known as a _trait bound_:
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarise());
}

// there are cases where we will want to use the trait bound syntax, e.g.,
// pub fn notify(item1: &impl Summary, item2: &impl Summary) { }
// will allow the item1 and item2 to be different types that implement Summary, whereas
// pub fn notify<T: Summary>(item1: &T, item2: &T) { }
// requires item1 and item2 to be the same type (which implements Summary)

// -- Specifying multiple trait bounds with + --
// We may also want to specify more than one trait bound (i.e., have a type that implements multiple traits)
// We do so with the + syntax:
// pub fn notify(item: &(impl Summary + Display)) { }
// and with trait bounds:
// pub fn notify<T: Summary + Display>(item: &T) { }

// -- Clearer trait bounds with `where` clauses --
// Using many trait bounds can make function signatures hard to read, so instead of this:
// fn some_func<T:Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}
// we can write this:
// fn some_func<T, U>(t: &T, u: &U) -> i32
// where 
//     T: Display + Clone,
//     U: Clone + Summary,
// {0}

// -- Returning types that implement traits --
// We can also use the impl Trait syntax to specify that the return type is any type which implements a trait, e.g.:
fn returns_summarisable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks2"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
// n.b. you can only return a single type. a function that can switch between returning NewsArticle or Tweet would not compile..
// .. even though they both do impl Summary. There is a way to do it, covered in 17.2.

fn example_returns_summarisable() {
    let tweet = returns_summarisable();
    println!("Return summarisable! 1 new tweet: {}", tweet.summarise());
}


// -- Using trait bounds to conditionally implement methods --
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally ..
// .. for types that implement the specified traits... so methods only exist for the types with the correct traits.
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// so this trait bound means the methods within the impl block are only implemented if T has the traits Display+PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn conditional_methods() {
    let pair = Pair::new(1, 2);
    pair.cmp_display();
}

// We can also conditionally implement a trait for any type that implements another trait
// Implementations of a trait on any type satisfying the trait bounds are called _blanket implementations_ and are extensively used
// E.g., the ToString trait is implemented on any type that implements the Display trait:
// impl<T: Display> ToString for T {
//     // --snip--
// }
// We can then call .to_string on any type with the Display trait implemented

// Traits and trait bounds let us write code that reduces duplication, but specifies the behaviour we require
// The compiler uses the trait bound information to check all concrete types used provide the correct behaviour
// This means we're forced to fix potential type/behaviour errors before the code even runs - unlike in dynamically typed languages

// Examples from this section
pub fn examples() {
    defaults();
    example_returns_summarisable();
    conditional_methods();
}

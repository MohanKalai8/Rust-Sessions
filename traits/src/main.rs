use std::fmt::{format, Display,Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// we can also implement like this with trait bounds
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news-1! {}", item.summarize());
}

// we can also specify multiple trait bounds using +
pub fn notify2<T: Summary + std::fmt::Display>(item: &T) {
    println!("Breaking news-2! {}", item.summarize());
}

// clearer Traits bounds with where caluses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> u32 {
    1
} // this is not readable

fn some_function1<T, U>(t: &T, u: &U) -> u32
where
    T: std::fmt::Display,
    U: std::fmt::Debug,
{
    2
}

// Returning Types That implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("mohan"),
        content: String::from("of course, i am currently learning Rust"),
        reply: false,
        retweet: false,
    }
}
// But in the above fn you can't return different types(structs) like Tweet,NewsArticle by using if/else blocks. You can only able to return one type

// use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("mohan"),
        content: String::from("of course, i am currently learning Rust"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    notify1(&tweet);
}

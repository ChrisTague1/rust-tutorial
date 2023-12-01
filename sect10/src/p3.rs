use std::fmt::{Display, Debug};

// a custom trait
trait Summary {
    fn summarize(&self) -> String;
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    contents: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// could use (item: &(impl Summary + Display)) to make it require both traits
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// <T: Summary + Display>
fn another_notify<T: Summary>(item: &T) {
    println!("More breaking news! {}", item.summarize());
}

#[allow(dead_code)]
pub fn run() {
    let tweet = Tweet {
        username: String::from("chris.tague"),
        content: String::from("My first tweet!"),
        reply: false,
        retweet: false
    };

    notify(&tweet);
    another_notify(&tweet);
}






#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {
    println!("I have a better syntax");
}

#[allow(dead_code)]
fn some_function_2<T, U>(_t: &T, _u: &U)
where
    T: Display + Clone,
    U: Clone + Debug
{
    println!("I am the better syntax");
}


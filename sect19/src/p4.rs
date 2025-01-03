#![allow(dead_code)]

// FnOnce - I consume/take ownership of my environment, so I can only be called once
// FnMut = I mutate my environment, but don't take ownership
// Fn - I am normal; I don't mutate or take ownership of my environment
// fn - I am a type, not a trait

fn add_one(x: i32) -> i32 {
    x + 1
}

// fn implements all three of the Closure traits, so it is best to use the Closure traits
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/// Closure cannot be returned on their own
/// They must be wrapped in a Box or some kind of abstraction because they are unsized
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

pub fn run() {
    let answer = do_twice(add_one, 5);

    println!("The answer is {answer}");

    println!("Added one: {}", returns_closure()(1));
}

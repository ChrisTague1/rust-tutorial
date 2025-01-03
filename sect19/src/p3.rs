#![allow(dead_code)]

type Thunk = Box<dyn Fn() + Send + 'static>;

// implicitly has a Sized restriction
fn generic1<T>(_t: T) {}

// explicit Sized restriction
fn generic2<T: Sized>(_t: T) {}

// relaxing the Sized restriction
// the ?Sized syntax is only allowed for Sized
// If it is ?Sized it must be a reference
fn generic3<T: ?Sized>(_t: &T) {}

pub fn run() {
    let _f: Thunk = Box::new(|| println!("Hello!"));
}

use std::ops::Deref;

struct MyBox<T>(T);

#[allow(dead_code)]
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[allow(dead_code)]
pub fn run() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y is replaced with *(y.deref())
    
    // under the hood, rust will attempt to call deref
    // any number of times to make function types match
    //
    // All done at comp time so no runtime penalty for n coercions
    let name = MyBox::new(String::from("idiot"));
    hello(&name);
}

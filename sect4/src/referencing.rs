#[allow(dead_code)]
pub fn run() {
    run1();
    run2();
}

fn run2() {
    let reference_to_something: String = dangle();
    println!("{reference_to_something}");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn run1() {
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of `{}` is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    println!("Hello, world!");
    println!("{} day", 31);
    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");
    println!("{chris}, {nathan}, {jim}", chris="chris", nathan="nathan", jim="jim");
    println!("{} of {:b} people know binary, the rest don't", 1, 100);
    println!("{number:>width$}", number=1, width=6); // what, this doesn't seem obvious
    println!("{number:a>width$}", number=1, width=6);
    println!("{:0>width$}", 1, width=6);

    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is {pi:.4}");
}

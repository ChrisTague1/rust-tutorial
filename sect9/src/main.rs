use std::error::Error;
use std::fs::File;

mod p1;

// Box<dyn Error> means anything that implements the Error trait
fn main() -> Result<(), Box<dyn Error>> {
    // let v = vec![1, 2, 3];
    // v[99]; // this will panic, do 'RUST_BACKTRACE=1 cargo run' to see
    
    let _greetings_file = File::open("chris.txt")?;

    p1::run();

    Ok(())
}

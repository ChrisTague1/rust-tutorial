mod compound;
mod expressions;
mod control;

fn main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constants are computed at compile time

    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // println!("{THREE_HOURS_IN_SECONDS}");

    // let y = 5;
    // let y = y + 1;

    // {
    //     let y = y * 2;
    //     println!("y in the inner scope is {y}");
    // }

    // println!("The value of y is {y}");

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("Spaces: {spaces}");
    
    // Integers are sized 8 to 128 (doubling each time) 
    //  with an additional size type. These are all prefaced
    //  by i (integers can be negative) or u (unsigned)

    compound::compound();
    expressions::expressions();
    control::control();
}

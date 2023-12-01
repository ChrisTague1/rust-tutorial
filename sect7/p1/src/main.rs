// this is an import use Beans without garden::vegetable::Beans
use crate::garden::vegetable::Beans;
// this tells the compiler to go compile and include that code
pub mod garden;

fn main() {
    let plant = Beans {};
    println!("I am growing {:?}!", plant);
}

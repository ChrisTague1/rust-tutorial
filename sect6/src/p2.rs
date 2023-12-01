#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Iowa,
    Illinois,
    Texas
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {
    fn value_in_cents(&self) -> u8 { 
        println!("Hello, there!");

        match self {
            Coin::Penny => 1,
            Coin::Dime => 10,
            Coin::Nickel => 5,
            Coin::Quarter(state) => {
                println!("Quarter printed in {state:?}");
                25
            }
        }
    }
}

#[allow(dead_code)]
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

#[allow(dead_code)]
pub fn run() {
    let coin = Coin::Quarter(UsState::Iowa);
    println!("The value of the coind is {value}", value=coin.value_in_cents());

    let dice_roll = 9;
    match dice_roll {
        3 => println!("You got a three!"),
        7 => println!("That was a 7"),
        other => println!("The number {other} isn't a 3 or 7")
    }
}


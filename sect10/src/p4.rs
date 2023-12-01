#[allow(dead_code)]
pub fn run() {
    example_of_lifetime();
    longest_example();
}

fn longest_example() {
    let string1 = String::from("chris");
    let string2 = "nathan";

    let result = longest(string1.as_str(), string2);
    println!("The longest string was {}", result);

    let string3 = "chris again";

    {
        let string4 = "nathan again";
        let result = longest(string3, string4);

        println!("The longest one was {}", result);
    }

    let result = longest("hello", "goodbye");
    println!("{result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn example_of_lifetime() {
    let r;

    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }

    // err: borrowed value does not live long enough
    // println!("r: {}", r);
}


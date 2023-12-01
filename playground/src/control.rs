pub fn control() {
    the_if_statement(6);
    the_loop();
    label_loops();
    for_loops();
}

fn the_if_statement(x: i32) {
    let y = if x > 5 {5} else {0};
    println!("y is {y}");
}

fn the_loop() {
    let mut counter: u8 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn label_loops() {
    // must start with a single quote
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn for_loops() {
    let a = [1, 2, 5, 7, 2, 7, 5];

    // iterates through a collection
    for element in a {
        println!("the value is {element}");
    }
}

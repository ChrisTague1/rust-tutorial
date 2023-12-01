pub fn run() {
    let config_max = Some(3u8);
    match config_max {
       Some(max) => println!("Config max is {max}"),
       _ => ()
    }

    if let Some(max) = config_max { // <<pattern>> = <<expression>> {stuff if they match}
        println!("Config max is {max}");
    } else {
        println!("Looks like something went wrong getting the config max :(");
    }
}

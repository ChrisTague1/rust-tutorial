use std::fs::{self, File};
use std::io::{self, Read};

pub fn run() {
    let _username = match read_username_from_file() {
        Ok(username) => username,
        Err(_) => "error".to_string()
    };
    println!("{_username}");

    let _username = match better_read_username_from_file() {
        Ok(username) => username,
        Err(_) => "error".to_string()
    };
    println!("{_username}");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("chris.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn better_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("chris.txt")
}


#[allow(dead_code)]
pub fn run() {
    let s: String = String::from("hello world");
    let word: usize = first_word(&s);
    println!("The first word ends at {word}");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("{} {}", hello, world);

    let word: &str = new_first_word(&s);
    println!("The first word is {word}");
}

fn new_first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[allow(dead_code)]
pub fn run() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // does not work because s1 is invalidated to avoid double free
    // {
    //     let s1 = String::from("hello");
    //     let s2 = s1;
    //     println!("{}, world!", s1);
    // }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
    }
    // functions by default take ownership (like the first example), but you can get around this with a return value.

    {
        #[allow(unused_mut)]
        let mut s1 = String::from("hello");
        take_ownership(s1);
    }
}

#[allow(dead_code)]
fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string is dropped from memory

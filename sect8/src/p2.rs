#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn run() {
    let mut s = String::new();

    let data = "initial contents"; // data is a literal, not really a string
    let s = data.to_string(); // to_string comes from Display trait
    {
        let this_is_equivelant_to_the_above = String::from("initial contents");
    }

    { // strings can be updated, and take a slice as to not take ownership
        let mut s1 = String::from("foo");
        let s2 = "bar";

        s1.push_str(s2);

        println!("s2 is {s2}");

        s1.push('c'); // push just takes a character
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world");
        let s3 = s1 + &s2; // cannot use s1 after this point
    }

    { // can use format! to deal with complex string formatting, doesn't lose ownership
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");

        println!("{s1} and {s2} and {s3} make {s}");
    }

    {
        let hello = String::from("Hello, world!");
        let s: &str = &hello[0..3]; // traditional indexing is not supported, but this can be done (not
                                    // recommended)
                                    //
                                    // Note: String is a wrapper around a vector, an expandable
                                    // list
                                    // str is a reference to a part of a string. There are always
                                    // immutable

        println!("{hello}{s}");
    }

    { // iterating over strings
        for c in "hey".chars() { // can iterate over the chars
            println!("{c}");
        }

        for b in "hey".bytes() { // or the bytes, note that these might not always be equal
            println!("{b}");
        }
    }
}

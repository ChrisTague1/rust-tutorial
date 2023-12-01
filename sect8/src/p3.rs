use std::collections::HashMap;

pub fn run() {
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 5);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        // get returns Option<&V>
        // .copied makes it Option<V> (v is i32 in this example)
        // .unwrap_or gets the value from Option, or 0 if it is None
        
        println!("score: {score}"); // things implementing Copy trait are copied in,
                                    // otherwise the HashMap takes ownership
    }

    { //looping
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 5);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    { // overriding values
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 5);

        println!("{:?}", scores);
    }

    { // Entry enum
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(30);

        println!("{:?}", scores);
    }

    {
        let text = "hello world how is everyone doing today";

        let mut map = HashMap::new();

        for character in text.chars() {
            let count: &mut i32 = map.entry(character).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}

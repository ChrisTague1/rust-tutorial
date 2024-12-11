mod p_1_and_2 {
    #[allow(dead_code)]
    pub fn run() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }

        let _my_option: Option<usize> = None;
    }
}

mod p3p1 {
    #[allow(dead_code)]
    pub fn run() {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            _ => println!("Some number")
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default, x = {x:?}")
        }

        println!("At the end: x = {x:?}, y = {y}");

        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("Some number who cares")
        }

        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("Not one two three four or five")
        }

        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
}

mod p3p2 {
    struct Point {
        x: i32,
        y: i32,
    }

    #[allow(dead_code)]
    pub fn run() {
        let p = Point { x: 0, y: 7 };

        let Point {x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }
}

mod p3p3 {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    #[allow(dead_code)]
    pub fn run() {
        // let msg = Message::ChangeColor(Color::h);
        let numbers = (2, 4, 6, 8, 10);

        match numbers {
            (first, .., last) => {
                println!("First is {first}, last is {last}");
            }
        }
    }
}

fn main() {
    p3p3::run();
}

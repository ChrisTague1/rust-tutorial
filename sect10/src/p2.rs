use std::fmt::Display;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let int_point = Point { x: 5, y: 6 };
    let float_point = Point { x: 1.0, y: 0.9 };

    println!("{:?}", int_point);
    println!("{:?}", float_point);

    println!("{}", int_point.x());
    println!("Distance from origin: {}", float_point.distance_from_origin());

    int_point.cmp_display();

    let _point_point = Point {
        x: Point {
            x: 1,
            y: 2
        },
        y: Point {
            x: 3,
            y: 4
        }
    };

    // _point_point.cmp_display(); // won't compile because Point doesn't implement the right
    // things
}



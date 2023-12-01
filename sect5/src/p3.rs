#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self { // in this case, Self means Rectangle (alis for the impl type)
        Rectangle {
            width: size,
            height: size
        }
    }
}

pub fn run() {
    let rect = Rectangle {
        width: 2,
        height: 7
    };

    let rect2 = Rectangle {
        width: 1,
        height: 6
    };

    println!("The area is {area}", area=rect.area());

    if rect.width() {
        println!("The rectangle has a nonzero width of {}", rect.width);
    }
    
    println!("Can rectangle 1 hold rectangle 2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(5);
    println!("The square is {square:?}");
}
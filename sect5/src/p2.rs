#[allow(dead_code)]
pub fn run() {
    first();
    second();
    third();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[allow(dead_code)]
fn third() {
    let rect = Rectangle {
        width: 41,
        height: 12
    };

    dbg!(&rect);
    println!("Area is {}", area3(&rect));
    println!("{rect:?}");
    println!("{rect:#?}");
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[allow(dead_code)]
fn second() {
    let rect = (30, 50);
    println!("The area of the rectangle is {}", area2(rect));
}

#[allow(dead_code)]
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[allow(dead_code)]
fn first() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {}", area1(width, height));
}

#[allow(dead_code)]
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

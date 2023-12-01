pub fn compound() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // cannot change in
    let (x, y, z) = tup;
    println!("(x, y, z) = ({}, {}, {})", x, y, z);
    let five_hundred = tup.0;
    println!("500: {}", five_hundred);

    let _ar: [u8; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // [3, 3, 3, 3, 3];
}

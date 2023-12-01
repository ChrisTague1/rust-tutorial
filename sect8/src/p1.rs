#[allow(dead_code)]
pub fn run() {
    let v = vec![1, 2, 3, 4, 5]; // this works
    // let mut v = Vec::new(); // this works
    // v.push(1);
    
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element")
    }


    let mut v = vec![100, -5, 9];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("Item is {i}");
    }
}

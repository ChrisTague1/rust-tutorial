use std::thread;

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
pub fn run() {
    let list = vec![1, 2, 3];

    println!("Before closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("Before closure: {:?}", list);

    // --------------------------------------

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 4,
            height: 2,
        },
        Rectangle {
            width: 5,
            height: 9,
        },
    ];
    let mut count: u8 = 0;

    list.sort_by_key(|r| {
        count += 1;
        return r.width;
    });
    println!("{:#?}", list);
    println!("{}", count);
}

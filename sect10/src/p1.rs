#[allow(dead_code)]
pub fn run() {
    let number_list = vec![1, 2, 3, 4, 5, 6];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![5, 45, 2, 57, 2, 6, 43, 12, 0, 5];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['a', 'b', 'c', 'd'];
    let result = largest(&char_list);
    println!("The largest item is {result}");
}

// std::cmp::PartialOrd is necessary because of the > comparison
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


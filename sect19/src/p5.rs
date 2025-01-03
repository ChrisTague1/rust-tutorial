#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn run() {
    let arr = my_vec![1, 2, 3, 4, 5];
    
    for item in arr {
        println!("{item}");
    }
}

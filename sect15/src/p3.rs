struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

#[allow(dead_code)]
pub fn run() {
    let _c = CustomSmartPointer {
        data: String::from("My stuff")
    };

    let _d = CustomSmartPointer {
        data: String::from("More Stuff")
    };

    println!("They are both created now");
}

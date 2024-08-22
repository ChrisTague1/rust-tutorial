#[allow(dead_code)]
struct MyThing {
    data: String
}

#[allow(dead_code)]
struct MyStruct {
    field: MyThing
}

#[allow(dead_code)]
pub fn run() {
    { // This is so happy

        let b = Box::new(5);
        println!("b = {b}");
    }

    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    let another: Box<MyStruct>;

    {
        // thing is created on the stack
        let thing = MyStruct { field: MyThing { data: "Hello, world".to_string() } };
        // Two things happen:
        //  1. Space is allocated on the heap, and thing is bit by bit copied over to the heap
        //  2. The ownership of thing is moved to this box/another, so the print below this will
        //     fail
        //
        // Note: MyThing does not implement copy but is bit-for-bit copied (bfb). Some key differences
        // are:
        //  1. bfb only happens during moves; without the copy trait the original will never be
        //     valid
        another = Box::new(thing);

        // println!("{}", thing.field.data);
    }

    let what = another.field;
    println!("{}", what.data);
}

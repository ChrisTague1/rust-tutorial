// enum IpKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     kind: IpKind,
//     address: String
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("You called me");
    }
}

#[allow(dead_code)]
pub fn run() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // let home = IpAddr {
    //     kind: IpKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpKind::V6,
    //     address: String::from("::1")
    // };

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}
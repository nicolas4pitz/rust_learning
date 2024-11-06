use std::fmt::Debug;


enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChanceColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // method body would be defined here
    }
}

fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let mesage = Message::Write(String::from("hello"));
    mesage.call();
}

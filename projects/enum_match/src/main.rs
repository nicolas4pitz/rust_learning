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

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Option<T> {
    Some(T),
    None,
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }


fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    //let home = IpAddrKind::V4(127, 0, 0, 1);

    //let loopback = IpAddrKind::V6(String::from("::1"));

    let mesage = Message::Write(String::from("hello"));
    mesage.call();

    let din = Coin::Penny;
    //value_in_cents(din);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let valor = Option::Some(5);

    if let Option::Some(v) = valor {
        println!("Encontrado um valor: {}", v);
    } else {
        println!("Nenhum valor encontrado.");
    }

}

/// with values without constructors
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/// with parameters
enum IpAddr2 {
    V4(String),
    V6(String),
}

/// with different constructors
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

use std::net::{Ipv4Addr, Ipv6Addr};

enum IpAddr5 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn playground1() {
    let four: IpAddrKind = IpAddrKind::V4;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    DisposeAt { x: i32, y: i32 },
}

/// attaching code
impl Message {
    fn call1(&self) {

    }
}

fn playground2() {
    fn work_with_message(msg: Message) {
        match msg {
            x@ Message::Quit => { dbg!(x); }
            Message::Move { .. } => {}
            Message::Write(_) => {}
            Message::ChangeColor(_, _, _) => {}
            Message::DisposeAt { .. } => {}
        }
    }

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn cents_to_coin(x: u8) -> Option<Coin> {
    match x {
        1 => Some(Coin::Penny),
        5 => Some(Coin::Nickel),
        10 => Some(Coin::Dime),
        25 => Some(Coin::Quarter),
        _ => None
    }
}

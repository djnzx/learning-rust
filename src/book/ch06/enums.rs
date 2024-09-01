/// with values without constructors
enum IpAddrKind {
    V4,
    V6,
}

fn route(kind: IpAddrKind) {}

fn test1() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    route(four);
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
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
    Quit,                         // actually struct
    Move { x: i32, y: i32 },      // actually struct
    Write(String),                // actually struct
    ChangeColor(i32, i32, i32),   // actually struct
    DisposeAt { x: i32, y: i32 }, // actually struct
}

/// attaching code
impl Message {
    fn call1(&self) {}
}

#[test]
fn playground2() {
    fn work_with_message(msg: Message) {
        match msg {
            x @ Message::Quit => {
                dbg!(x);
            }
            Message::Move { .. } => {}
            Message::Write(text) if text == "hello" => {}
            Message::Write(_) => {}
            Message::ChangeColor(_, _, _) => {}
            Message::DisposeAt { .. } => {}
        }
    }
}

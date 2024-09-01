use std::fmt::Display;
use std::fmt::Formatter;

// 1. definition
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[test]
fn struct_creation() {
    let u = User {
        active: true,
        username: String::from("Alex"),
        email: String::from("alexr@google.com"),
        sign_in_count: 1,
    };
    // structs are immutable (by default)
    // u.active = false;

    dbg!(u); // Debug trait required
}
#[test]
fn struct_creation_mutable() {
    let mut u = User {
        active: true,
        username: String::from("Alex"),
        email: String::from("alexr@google.com"),
        sign_in_count: 1,
    };
    u.active = false;

    dbg!(u); // Debug trait required
}
#[test]
fn struct_copy() {
    let u1 = User {
        active: true,
        username: String::from("Alex"),
        email: String::from("alexr@google.com"),
        sign_in_count: 1,
    };
    let u2 = User {
        username: String::from("Jim"),
        email: String::from("jim@yahoo.com"),
        ..u1
    };
    dbg!(u1);
    dbg!(u2);
}

fn mk_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // corresponding naming can be omitted
        email: email,
        sign_in_count: 1,
    }
}

#[test]
fn test2_creation() {
    let u = mk_user(String::from("alexr@google.com"), String::from("Alex"));
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn playground1() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1); // requires Display

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{:?}", user2); // requires Debug
}

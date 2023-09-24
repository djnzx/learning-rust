use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email: email,
        sign_in_count: 1,
    }
}

fn playground1() {

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1);   // requires Display

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{:?}", user2); // requires Debug


}

/// without names
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

/// with names
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// attaching methods
impl Rectangle {

    /// with context
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// static w/o context
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}


fn playground2() {
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    let subject = AlwaysEqual;
    let r = Rectangle {
        width: 10,
        height: 10
    };

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        r.area()
    );
    let sq = Rectangle::square(3);
    dbg!(&r);

}
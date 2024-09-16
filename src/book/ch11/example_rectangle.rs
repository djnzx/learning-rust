/// since we declared our static data as public
/// we must declare this struct public
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// things know at the compile time
/// can be put into the static scope
mod data {
    use super::Rectangle;

    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref larger: Rectangle = Rectangle { width: 8, height: 7 };
        pub static ref smaller: Rectangle = Rectangle { width: 5, height: 1 };
    }
}

use data::larger;
use data::smaller;

#[test]
fn larger_can_hold_smaller() {
    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    assert!(!smaller.can_hold(&larger));
}

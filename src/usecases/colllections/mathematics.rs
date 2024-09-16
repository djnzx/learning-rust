use std::cmp::{min_by, Ordering};
use std::ops::Add;

mod global_scope {
    pub static X: u32 = 1;
    pub static Y: u32 = 2;
}

#[test]
fn code1() {
    let x = f32::sqrt(4f32);
    let x = 2_i32.pow(3);
    dbg!(x);
}

#[test]
fn code2() {
    let x = i32::min(5, 6);
    let x = 5.min(6);
    let x = 1 + 2;
    let x = 1.add(2);
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    count: u32,
    total: f32,
}

impl Add for Cart {
    type Output = Cart;

    fn add(self, rhs: Self) -> Self::Output {
        Cart {
            count: self.count + rhs.count,
            total: self.total + rhs.total,
        }
    }
}

#[test]
fn adding_cart() {
    let cart1 = Cart { count: 6, total: 50.64 };

    let cart2 = Cart { count: 3, total: 20.32 };

    let cart3 = cart1 + cart2;

    println!("{:?}", cart1);
    println!("{:?}", cart2);
    println!("{:?}", cart3);
}

impl PartialEq<Self> for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total && self.count == self.count
    }
}

#[test]
fn comparing_carts_are_equal() {
    let cart1 = Cart { count: 6, total: 50.64 };

    let cart2 = Cart { count: 6, total: 50.64 };

    let cart3 = Cart { count: 3, total: 20.32 };

    let areEq = cart1 == cart2;

    assert_eq!(cart1, cart2);
    assert_ne!(cart1, cart3);
}

impl PartialOrd<Self> for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total.partial_cmp(&other.total)
    }
}

impl Eq for Cart {}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total
            .partial_cmp(&other.total)
            .unwrap()
    }
}

#[test]
fn comparing_cart() {
    let cart1 = Cart { count: 2, total: 50.64 };
    let cart2 = Cart { count: 3, total: 20.32 };

    let c31 = min_by(cart1, cart2, |c1, c2| c1.count.partial_cmp(&c2.count).unwrap());
    let c32 = cart1.min(cart2);
    let c32 = Cart::min(cart1, cart2);

    println!("{:?}", c31); // min by count: Cart { count: 2, total: 50.64 }
    println!("{:?}", c32); // min by total: Cart { count: 3, total: 20.32 }
}

#[test]
fn code5() {
    impl PartialEq for F {
        fn eq(&self, other: &Self) -> bool {
            f64::abs(self.value - other.value) <= 0.0001
        }
    }

    println!("{}", F { value: 1.0001 } == F { value: 1.0003 }); // false
    println!("{}", F { value: 1.0001 } == F { value: 1.0002 }); // true
}

pub struct F {
    pub value: f64,
}

#[test]
fn code6() {}

use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //                             () -> self.most_stocked()
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[test]
fn test_with_pref() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let pref = Some(ShirtColor::Red);
    let got = store.giveaway(pref);
    println!("The user with preference {:?} gets {:?}", pref, got);
}

#[test]
fn test_without_pref() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let pref = None;
    let got = store.giveaway(pref);
    println!("The user with preference {:?} gets {:?}", pref, got);
}

// function can be generic
// closure must be with generic resolved
fn id<A>(a: A) -> A {
    a
}

#[test]
fn different_syntax() {
    // 1. function definition
    fn add_one_v0(x: u32) -> u32 {
        x + 1
    }
    // closure definition with explicit type
    let add_one_v1: fn(u32) -> u32 = |x: u32| -> u32 { x + 1 };
    let add_one_v2: fn(u32) -> u32 = |x| -> u32 { x + 1 };
    let add_one_v3: fn(u32) -> u32 = |x| x + 1;

    // closure definition with type inferred
    let add_one_v4 = |x: u32| -> u32 { x + 1 };

    // will work only when type can be inferred
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // infer type based on the later call
    // type is fixed after the first use
    let identity = |x| x;

    let x = identity(3);
    let y = id(5);
    let y = id(PI);
}

// capture_references in three ways:
// - borrowing immutably
// - borrowing mutably
// - taking ownership

#[test]
fn capture_immutable_reference() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

#[test]
fn capture_mutable_reference() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // must be mut, since borrow a mutable
    let mut borrows_and_mutates = || list.push(7); // here we borrowed

    // can't do it, since it's already borrowed
    // because println tries to borrow
    // println!("Before calling closure2: {:?}", list);
    borrows_and_mutates(); // borrow released
    println!("After calling closure: {:?}", list);
    list.push(11);
    println!("After calling closure: {:?}", list);
}

#[test]
fn explicit_move() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // `move` to force the closure for the thread to take ownership of list
    // thread can live longer than the parent function, that's why we need `move`
    let t = thread::spawn(move || println!("From thread: {:?}", list));

    t.join().unwrap();
}

// FnOnce - applies to closures that can be called once. All closures implement at least this trait
// FnMut - applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
// Fn - applies to closures that don’t move captured values out of their body and that don’t mutate captured values

// `sort_by_key` defined on slices, differs from `unwrap_or_else`
// and why
// `sort_by_key` uses `FnMut` instead of `FnOnce` (`unwrap_or_else`)

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
pub fn use_as_fn_once() {
    // to sort, list must be mutable
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // The reason sort_by_key is defined to take an FnMut closure is that
    // it calls the closure multiple times: once for each item in the slice.
    // The closure |r| r.width doesn't capture, mutate,
    // or move out anything from its environment, so it meets the trait bound requirements.
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    list.sort_by_key(|r| r.height);
    println!("{:#?}", list);
}

#[test]
pub fn use_as_fn_mut1a() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value.clone()); // here we copy data !!!
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}", sort_operations);
}

#[test]
pub fn use_as_fn_mut1b() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations: Vec<&str> = vec![];
    let binding = String::from("by key called");
    let value = binding.as_str();

    list.sort_by_key(|r| {
        sort_operations.push(value); // here we copy link and this is fine
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}", sort_operations);
}

#[test]
pub fn use_as_fn_mut2() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut count = 0;

    list.sort_by_key(|r| {
        count += 1;
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}", count);
}

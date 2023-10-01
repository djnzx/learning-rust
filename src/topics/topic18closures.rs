use std::ffi::CString;
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

pub fn main1() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

pub fn main2() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v21 = |x: u32| -> u32 { x + 1 };
    let add_one_v22: fn(u32) -> u32 = |x: u32| -> u32 { x + 1 };
    // will work only when type can be inferred
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    fn identity_fn<A>(a: A) -> A {
        a
    }

    let identity = |x| x; //typed after the first use
    let s = identity(String::from("hello"));
    let n = identity(5.to_string());

    dbg!(s);
    dbg!(n);
}

pub fn main3() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_and_mutates = || list.push(7); // here we borrowed

    // println!("Before calling closure: {:?}", list); // can't do it, since it's already borrowed
    borrows_and_mutates(); // borrow released
    println!("After calling closure: {:?}", list);
}

pub fn main4() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // `move` to force the closure for the thread to take ownership of list
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main5() {
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

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    list.sort_by_key(|r| r.height);
    println!("{:#?}", list);
}

pub fn main() {
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

    let mut num_sort_operations = 0;
    // let mut sort_operations: Vec<String> = vec![];
    // let value = String::from("by key called");

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        //sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}", num_sort_operations);
}

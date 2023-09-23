use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::io;
use ferris_says::say;
use std::io::{stdout, BufWriter, Stdin, Error};
use std::iter::Enumerate;
use std::ops::Range;
use std::slice::Iter;
use rand::Rng;

fn main() {

    fn add(a: i8, b: i8) -> i8 {
        a + b
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    'there: loop {
        break 'there;
    }

    // let a: u8 = 255;
    // let b: u8 = 1;
    // let c: u8 = a+b;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let x = 1;
    // shadowing, not mutating
    let x = x + 1;

    // tuple
    let t = (1, 2.5, true, "hello");
    let fst = t.0;
    let (q,w,e,r) = t;
    let a = [1, 2, 3, 10, 11];
    let a: [i8; 5] = [1, 2, 3, 10, 11];
    let s1 = &a[2..3];
    let s2 = &a[..3];
    let s3 = &a[2..];


    let a = [5; 10]; // 5,5,5,5,5,5,5,5,5,5

    let l = a.len();
    for a in a {
        println!("{a}")
    }
    let r: Range<i32> = 1..4;

    {
        let s1: &str = "abc";
        let s2: String = String::from("hello");
        println!("{s1}");
        println!("{s2}");
        let mut s3 = String::from("hello");
        s3.push_str(", world!");
        println!("{}", s3);
    }
    {
        let s1 = String::from("hello");
        let s2 = s1;         // move occurs because `s1` has type `String`, which does not implement the `Copy` trait.
        //println!("{}, world!", s1); // value borrowed here after move

        let s3 = s2.clone();
        println!("{}, world!", s2);
        println!("{}, world!", s3);
    }

    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let mut guess = String::new();

    let in0: Stdin = io::stdin();

    in0
        .read_line(&mut guess)
        .expect("Failed to read line")
    ;

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    println!("You guessed: {guess}");

    let x = match guess.cmp(&guess) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };

    let mut x = 0;

    loop {
        println!("{x}");
        x += 1;
        match x.cmp(&5) {
            Ordering::Less => {}
            Ordering::Equal => {println!("Win!") ; break}
            Ordering::Greater => {}
        }
    }

    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        break;
    }

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);            // s's value moves into the function...
                                              // ... and so is no longer valid here

    let x = 5;                           // x comes into scope

    makes_copy(x);                // x would move into the function,
                                              // but i32 is Copy, so it's okay to still
                                              // use x afterward

    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    {
        let mut s: String = String::from("hello");

        let r1 = &mut s;
        // let r2 = &s;     // borrowing
        // let r2 = &mut s; // only one is allowed

        println!("{}, ", r1);
        // println!("{}, {}", r1, r2);
    }

    // it returns index of space
    fn first_word(s: &String) -> usize {
        let bytes: &[u8] = s.as_bytes(); // ??? copy
        let it: Iter<u8> = bytes.iter();
        let en: Enumerate<Iter<u8>> = it.enumerate(); // withIndex

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
    {
        let s = String::from("hello world");

        // slicing doesn't take a memory
        let hello = &s[0..5];
        let world = &s[6..11];
        let s1 = &s[..5];
        let s2 = &s[6..];



    }
    fn first_word_v2(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // impl Display for User {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //         todo!()
    //     }
    // }
    // we must name fields
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email: email,
            sign_in_count: 1,
        }
    }
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // fn update_email(user: &User, new_email: String) -> User {
    //     User {
    //         email: new_email,
    //         ..user.clone()
    //     }
    // }
    // println!("{}", user1);
    println!("{:?}", user2);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let r = Rectangle {
        width: 10,
        height: 10
    };

    // attaching methods
    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        // static w/o context
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        r.area()
    );
    let sq = Rectangle::square(3);
    dbg!(&r);
}

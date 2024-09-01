/// Each value in Rust has an owner.
/// There can only be one owner at a time.
/// When the owner goes out of scope, the value will be dropped.
#[test]
fn own1() {
    /// literal, immutable, pointer by default
    let s1 = "hello"; // fast, efficient, size known in compile time
    let s2 = String::from("hello");
    // can be mutable
    let mut s3 = String::from("hello"); // creates NEW from literal
                                        // append
    s3.push_str(", world!");
    dbg!(s3);
    // drop(s3); // no more s3 here
}

#[test]
fn own2() {
    let s1 = String::from("hello");
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // https://doc.rust-lang.org/book/img/trpl04-02.svg
    let s2 = s1; // copying only pointer here
                 // move occurs because `s1` has type `String`, which does not implement the `Copy` trait.
    /// dbg!(s1); // no more s1 (moved)
    let s3 = s2.clone(); // really deep copy
    dbg!(s2);
    dbg!(s3);
}

fn takes_ownership(s: String) {
    // some_string comes into scope
    println!("{}", s);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(x: i32) {
    // some_integer comes into scope
    println!("{}", x);
} // Here, some_integer goes out of scope. Nothing special happens.

#[test]
fn own3() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    dbg!(x);
    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    {
        let mut s: String = String::from("hello");

        let r1 = &mut s;
        // let r2 = &s;     // borrowing
        // let r2 = &mut s; // only one is allowed

        println!("{}, ", r1);
        // println!("{}, {}", r1, r2);
    }
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

#[test]
fn own4() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

#[test]
fn own5() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

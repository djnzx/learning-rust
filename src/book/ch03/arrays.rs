use std::ops::Range;

#[test]
fn creating() {
    // type + length
    let a = [10, 20, 30, 40, 50];
    let b: [i32; 5] = [10, 20, 30, 40, 50];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // duplicate elements
    let xs: [i32; 5] = [10; 5];
    dbg!(xs);
    /// mutable
    let mut xs: [i32; 5] = [10; 5];
}

#[test]
fn accessing() {
    /// immutable
    let xs = [10, 20, 30, 40, 50];

    let x = xs[2]; // 30

    /// slices take 0 time
    let slice1 = &xs[2..3]; // [30]
    let slice2 = &xs[..2]; // [10, 20]
    let slice3 = &xs[3..]; // [40, 50]
    /// let slice4 = &xs[3..10]; // panic

    /// length
    let len = slice1.len();

    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);
}

#[test]
fn iteration() {
    let xs = [10, 20, 30, 40, 50];

    for x in xs {
        println!("{x}")
    }
}

fn ranges() {
    /// range 1..<4
    let r1: Range<i32> = 1..4;
    dbg!(r1);
    /// range 1..4
    let r2 = 1..=4;
    dbg!(r2);
}

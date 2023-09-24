use std::ops::Range;

fn playground() {
    /// mutable
    let mut xs: [i32; 5] = [10; 5];

    /// immutable
    let xs = [10,20,30,40,50];
    let xs: [i32; 5] = [10,20,30,40,50];

    /// slices
    let slice1 = &xs[2..3]; // [30]
    let slice2 = &xs[..2];  // [10, 20]
    let slice3 = &xs[3..];     // [40, 50]

    /// range
    let r: Range<i32> = 1..4;

    /// length
    let len = slice1.len();

    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);

    /// iteration
    for x in xs {
        println!("{x}")
    }

}
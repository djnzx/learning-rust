use std::cmp::Ordering;

pub fn print_usize() {
    let size_of_usize = std::mem::size_of::<usize>();
    println!("Size of usize on this platform: {} bytes", size_of_usize); // i7 = 8 bytes = u64
}

fn playground() {
/// primitive data types
/// - unsigned: u8 / u16 / u32 / u64 / u128
/// - signed:   i8 / i16 / i32 / i64 / i128
/// - usize     platform dependent: u32/u64
/// - float     f32
/// - double    f64
/// - bool
/// - char
/// type inference
/// - explicit type
/// - literal notation

    let x = true;
    let x = 1.5;
    let x = 1.5f32;

    let x = 123;
    let x = 123u8;
    let x = 123i16;

    let c = 'z';
    let c: char = 'ℤ';
    let c = '😻';

    /// compare

    if x > 2 {
        println!("biggr")
    } else {
        println!("smallr")
    }

    /// if returns!
    let r: &str = if x > 2 {
        "biggr"
    } else {
        "smallr"
    };

    /// matching
    let x = 5;
    let y = 10;

    /// matching also returns
    /// curly braces
    let r = match x.cmp(&y) {
        Ordering::Less => { println!("Too small!") }
        Ordering::Equal => { println!("Too big!") }
        Ordering::Greater => { println!("You win!") }
    };

    /// or commas
    match x.cmp(&y) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Too big!"),
        Ordering::Greater => println!("You win!"),
    }

}
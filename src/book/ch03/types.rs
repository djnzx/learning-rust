use std::cmp::Ordering;

#[test]
pub fn print_usize() {
    let size_of_usize = std::mem::size_of::<usize>();
    println!("Size of usize on this platform: {} bytes", size_of_usize); // i7 = 8 bytes = u64 = long
}

fn constants() {
    // const don't infer the type, must be specified explicitly
    const X: u32 = 3;
}

fn shadowing() {
    let x = 1;
    let x = x + 1;
    let x = 2.5;
    let x = "   ";
    let x = x.len();
}
#[test]
fn scope() {
    let x = 1;
    println!("outer x={}", x);
    {
        let x = x + 1;
        println!("inner x={}", x);
    }
    println!("outer x={}", x);
}

fn inc(x: u8) {
    let x = x + 1; // panic when overflow, but not in rustc --release data loss
    println!("{x}");
}

// #[test]
fn one() {
    let x = panic!();
}

#[test]
fn primitives() {
    /// - unsigned: u8 / u16 / u32 / u64 / u128
    /// - signed:   i8 / i16 / i32 / i64 / i128
    /// - usize     platform dependent: u32/u64
    /// - float     f32
    /// - double    f64
    /// - bool
    /// - char
    // type inference
    let x = true;
    // - explicit type
    let x: f64 = 1.5;
    // - literal notation
    let x = 1.5f32;

    let x = 123;
    let x = 123u8;
    let x = 123i16;
    let card = 1234_5678_9012_3456u64;
    println!("z={card}");
    let z = card as u32; // data loss
    println!("z={z}");
    let byte: u8 = 0xFF; // hex
                         // will fail in runtime
                         // inc(byte);
                         // let byte = byte + 1; // compile error if literal

    let byte = 0o123; // octal
    let byte = 0b010101; // bin
    let byte = b'Z'; // byte only for ASCII

    // char is UTF, string is UTF up to four bytes
    let c = 'z';
    let c: char = 'ℤ';
    let c = '😻';
    // https://en.wikipedia.org/wiki/Two%27s_complement
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn explicit_handling() {
    u32::wrapping_add(1, 2);
    u32::checked_add(1, 2);
    u32::overflowing_add(1, 2);
    u32::saturating_add(1, 2);
}

fn parsing() {
    let x = "123".parse::<u32>();
}

fn comparing() {
    let x = 123i16;
    /// compare
    if x > 2 {
        println!("bigger")
    } else {
        println!("smaller")
    }

    /// if returns!
    let r: &str = if x > 2 { "bigger" } else { "smaller" };

    /// matching
    let x = 5;
    let y = 10;

    /// matching also returns
    /// curly braces
    /// () = Unit
    let r = match x.cmp(&y) {
        Ordering::Less => {
            println!("Too small!")
        }
        Ordering::Equal => {
            println!("Too big!")
        }
        Ordering::Greater => {
            println!("You win!")
        }
    };

    /// or commas
    match x.cmp(&y) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Too big!"),
        Ordering::Greater => println!("You win!"),
    }
}

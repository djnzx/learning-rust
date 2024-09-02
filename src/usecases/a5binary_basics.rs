#[test]
fn some_numbers() {
    println!("  -3 {:0>8b}", -3i8);
    println!("  -2 {:0>8b}", -2i8);
    println!("  -1 {:0>8b}", -1i8);
    println!("   0 {:0>8b}", 0i8);
    println!("   1 {:0>8b}", 1i8);
    println!("   2 {:0>8b}", 2i8);
    println!("   3 {:0>8b}", 3i8);
    println!("   4 {:0>8b}", 4i8);
    println!(" ...........");
    println!(" 124 {:0>8b}", 124i8);
    println!(" 125 {:0>8b}", 125i8);
    println!(" 126 {:0>8b}", 126i8);
    println!(" 127 {:0>8b}", 127i8);
    println!("-128 {:0>8b}", -128i8);
    println!("-127 {:0>8b}", -127i8);
    println!("-126 {:0>8b}", -126i8);
    println!("-125 {:0>8b}", -125i8);
}

#[test]
fn test_2_binary_rep() {
    let x = 5_u8;
    println!("{:0>8b}", x);
    let y = 7_u8;
    println!("{:0>8b}", y);
    let z = x + y;
    println!("{:0>8b}", z);
}

#[test]
fn test_2_hex_rep() {
    let x = 63_u8;
    println!("0x{:0>2x}", x);
    let x = 63_u8;
    println!("0x{:0>2X}", x);
}

#[test]
fn test_2_binary_rep_negative() {
    let x = 5;
    println!("{:0>32b}", x);
    let y = -5;
    println!("{:0>32b}", y);
    let z = x + y;
    println!("{:0>32b}", z);
    // 00101 - 5
    // 11010 - inverted
    // 11011 - 1 added
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
fn test_4_string_type_name() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

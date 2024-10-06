use num::ToPrimitive;

#[test]
fn code1() {
    let x = -1;
    let y = x as u8;
    println!("{}", y);

    let x = 123.456;
    let y = x as u8;
    println!("{}", y);

    let x = 97.123;
    let y = x as u8 as char;
    println!("{}", y); // `a`

    //https://www.utf8-chartable.de
    let x = 'Ы';
    let y = x as u16;
    let b1 = (y >> 8) as u8;
    let b2 = y as u8;
    println!("{0:} {0:x}", y);
    println!("{:08b} {:08b}", b1, b2);
    // actual bytes are: [D0,       AB      ]
    // actual bytes are: [208,      171     ]
    // actual bytes are: [11010000, 10101011]
    //                    ---       --
    // UTF code of meaningful bits are:
    //                       10000    101011
    //                       1067
    //                       042b
    //
    //
    // 1067 42b
    let b = 0b_10000_101011_u16;
    println!("{}", b);
}

#[test]
#[allow(overflowing_literals)]
fn assign() {
    let x = 1000u8;
    println!("{x}");
}

#[test]
fn rust145() {
    assert_eq!(300.123 as u8, u8::MAX);
    assert_eq!(-300.123 as u8, u8::MIN);
    assert_eq!((-300.123).to_u8(), None);
    assert_eq!((-300.123).to_i16(), Some(-300));
}

#[test]
fn inside_unsafe() {
    unsafe {
        let x = 300.0_f32.to_int_unchecked::<u8>();
        println!("{x}");
        assert_eq!(x, 44); // 300 - 256 = 44
    }
}

fn mk_real_bytes_2(utf8code: u16) {
    let b1_mask = 0b00000000_000111111_u16;
    let b1_data = (utf8code & b1_mask) as u8;
    let b1_prefix = (0b10 << 6) as u8;
    let b1 = b1_data | b1_prefix;

    let b0_prefix = (0b110 << 5) as u8;
    let b0_mask = 0b00000111_11000000_u16;
    let b0_data = ((utf8code & b0_mask) >> 6) as u8;

    let b0 = b0_data | b0_prefix;

    println!("{} {}", b0, b1);
    println!("{:x} {:x}", b0, b1);
}

#[test]
fn code2() {
    mk_real_bytes_2(0x042b); // "Ы"
}

#[test]
fn code3() {}

#[test]
fn code4() {}

#[test]
fn code5() {}

#[test]
fn code6() {}

fn show_separator() {
    println!("{:->20}", "");
}

fn rep_bin(x: u8) -> String {
    // let bits = x.to_bits();
    match x {
        n => format!("{:0>8b} | {0:3}", n),
    }
}

fn show1(a: u8) {
    println!("a = {}", rep_bin(a));
}

fn show2(a: u8, b: u8) {
    println!("a = {}", rep_bin(a));
    println!("b = {}", rep_bin(b));
}

fn show3(a: u8, b: u8, c: u8) {
    println!("a = {}", rep_bin(a));
    println!("b = {}", rep_bin(b));
    println!("c = {}", rep_bin(c));
}

#[test]
fn inversion() {
    let a: u8 = 1;
    let b: u8 = !a;
    show_separator();
    println!("logical inversion: !");
    show_separator();
    show2(a, b);
}

#[test]
fn shift_left() {
    let a: u8 = 8;
    let b: u8 = a << 1;
    let c: u8 = a << 2;
    show3(a, b, c);
}

#[test]
fn shift_right() {
    let a: u8 = 8;
    let b: u8 = a >> 1;
    let c: u8 = a >> 2;
    show3(a, b, c);
}

#[test]
fn logical_shift() {
    show_separator();
    println!("logical shift right: >>");
    show_separator();
    shift_right();
    show_separator();
    println!("logical shift left: <<");
    show_separator();
    shift_left();
    show_separator();
}

#[test]
fn logical_or_1() {
    let a: u8 = 96;
    let b: u8 = 3;
    let c = a | b;
    show3(a, b, c);
}

#[test]
fn logical_or_2() {
    let a: u8 = 5;
    let b: u8 = 1;
    let c = a | b;
    show3(a, b, c);
}

#[test]
fn logical_or_3() {
    let a: u8 = 5;
    let b: u8 = 3;
    let c = a | b;
    show3(a, b, c);
}

#[test]
fn logical_or() {
    println!("logical OR: |");
    show_separator();
    logical_or_1();
    show_separator();
    logical_or_2();
    show_separator();
    logical_or_3();
}

#[test]
fn logical_and_1() {
    let a: u8 = 1;
    let b: u8 = 2;
    let c = a & b;
    show3(a, b, c);
}
#[test]
fn logical_and_2() {
    let a: u8 = 2;
    let b: u8 = 3;
    let c = a & b;
    show3(a, b, c);
}

#[test]
fn logical_and_3() {
    let a: u8 = 3;
    let b: u8 = 6;
    let c = a & b;
    show3(a, b, c);
}

#[test]
fn logical_and() {
    println!("logical AND: &");
    show_separator();
    logical_and_1();
    show_separator();
    logical_and_2();
    show_separator();
    logical_and_3();
}

#[test]
fn logical_xor_1() {
    let a: u8 = 3;
    let b: u8 = 6;
    let c = a ^ b;
    show3(a, b, c);
}

#[test]
fn logical_xor_2() {
    let a: u8 = 255;
    let b: u8 = 32;
    let c = a ^ b;
    show3(a, b, c);
}

/// the same way we have:
/// &=
/// |=
/// ^=
/// >>=
/// <<=
#[test]
fn logical_and_eq() {
    let mut a: u8 = 5;
    show1(a);

    let b: u8 = 1;
    show1(b);

    a &= b;
    show1(a);
}

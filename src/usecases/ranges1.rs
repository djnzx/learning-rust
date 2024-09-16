use syn::Expr::Range;

#[test]
fn code1() {
    let v1 = 251_u16 + 5; // type of '5' is inferred to align
}

#[test]
fn code2() {
    let mut x: i32 = 0;

    for i in -3..2 {
        x += i;
    }

    assert_eq!(x, -5);
}

#[test]
fn code3() {
    for c in 'a'..='z' {
        print!("{} ", c as u8);
    }
}

#[test]
fn code4() {
    let r1 = 1..5;
    let r2 = 1..=5;
    let r3 = 3..; // infinite
    let r4 = ..5; // has no iterator
    let r5 = ..=5; // has no iterator

    r1.for_each(|i| print!("{} ", i)); // 1 2 3 4
    println!();
    r2.for_each(|i| print!("{} ", i)); // 1 2 3 4
    println!();
}

#[test]
fn code5() {
    let x = 9.6 / 3.2;
    assert_eq!(x, 2.9999999999999996);
}

#[test]
fn code6() {}

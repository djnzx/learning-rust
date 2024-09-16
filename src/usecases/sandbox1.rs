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
    let _ = 1;
}

#[test]
fn code5() {}

#[test]
fn code6() {}

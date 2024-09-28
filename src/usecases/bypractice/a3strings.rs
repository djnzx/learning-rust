#[test]
fn code1() {
    let s = "Ru\x73\x74!";
    println!("{}", s);

    let s = "\u{211D}"; // ℝ
    println!("{}", s);

    let s = "multi\
                  line";
    println!("{}", s);

    // raw
    let s = r"Ru\x73\x74!";
    println!("{}", s);
}

#[test]
fn empty_slice() {
    let s = "abc";
    let x = &s[0..0];
    println!("`{}`", x);
}

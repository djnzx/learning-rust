fn usebox(x: &str) {
    println!("{}", x);
}

#[test]
fn boxing() {
    // static area (stack)
    let x1 = "hello";
    // heap allocated
    let x2: Box<str> = x1.into();
    usebox(&x1);
    usebox(&x2);
}

#[test]
fn another_syntax() {
    let mut s = String::new();
    s += "abc";
    println!("{}", s);
}

#[test]
fn converting1() {
    let s = "abc";
    let s1 = String::from(s);
    let s2 = s.to_owned(); // trait ToOwned
    let s3 = s.to_string(); // trait ToString
}

#[test]
fn converting2() {
    let s = String::from("abc");
    let s1 = s.as_str();
    let s2 = &s;
}

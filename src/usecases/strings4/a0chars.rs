#[test]
fn size_char() {
    let c1 = 'a';
    let c2 = 'я';
    let c3 = 'स';
    let c4 = '💡';

    println!("{}", size_of_val(&c1)); // 4
    println!("{}", size_of_val(&c2)); // 4
    println!("{}", size_of_val(&c3)); // 4
    println!("{}", size_of_val(&c4)); // 4
}

#[test]
fn string_len() {
    let s1 = "a";
    let s2 = "я";
    let s3 = "स";
    let s4 = "💡";

    println!("{}", s1.len()); // 1
    println!("{}", s2.len()); // 1
    println!("{}", s3.len()); // 1
    println!("{}", s4.len()); // 1
}

#[test]
fn size_string() {
    let s1 = "a";
    let s2 = "я";
    let s3 = "स";
    let s4 = "💡";

    println!("{}", s1.bytes().len()); // 1
    println!("{}", s2.bytes().len()); // 2
    println!("{}", s3.bytes().len()); // 3
    println!("{}", s4.bytes().len()); // 4
}

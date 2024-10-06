// https://www.youtube.com/watch?v=BpPEoZW5IiY&t=27579s
fn move_ownership(s: String) {}

#[test]
fn code1() {
    let mut s = String::from("hello");
    s.push_str(" world");
    s.push('!');
    move_ownership(s.clone());
    assert_eq!(s, "hello world!");
}

/// String is a Vec<u8>
/// &str is a &[u8]

#[test]
fn code2() {
    let mut s = String::from("hello world");
    let slice1 = s.as_str();
    let slice2 = &s[..5];
    // this works only due to the fact mutable reference is only one
    let slice3 = &mut s;
    // what is the difference between slice3 and slice31
    // let mut slice31 = s;
    slice3.push_str(" !!!");
    println!("{}", slice3);
    println!("{}", s);
}

#[test]
fn code3() {
    let mut s = String::from("Привіт");
    println!("{}", s.len());
    for (i, c) in s.char_indices() {
        // bytes as offsets
        println!("index: {i}, char: {c}");
    }
}

#[test]
fn code4() {
    let s = "नमस्ते";
    let xs = s
        .chars()
        .skip(2)
        .take(1)
        .collect::<String>(); // here we have FromIterator trait
    print!("{:?}", xs);
}

#[test]
fn code5() {
    let mut s = String::new();
    let v = vec![104u8, 101, 108, 108];
    let s1 = String::from_utf8(v).unwrap();
    s.push_str(&s1);
    println!("{}", s);
}

#[test]
// 0,8,16,32,54,128...
fn capacity_grows_by_power_of2() {
    let mut s = String::new();
    println!("capacity:{}", s.capacity());
    for _ in 0..1000 {
        s.push('a');
        println!("capacity:{}", s.capacity());
    }
}

#[test]
// capacity still grows by starting from given, not 8
fn preallocate_special_size() {
    let mut s = String::with_capacity(500);
    println!("capacity:{}", s.capacity());
    for _ in 0..1000 {
        s.push('a');
        println!("capacity:{}", s.capacity());
    }
}

// String takes:
// - pointer to heap
// - len
// - capacity
// 8 * 3 = 24 bytes + the size of data

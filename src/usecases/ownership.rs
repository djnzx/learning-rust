/// 1. each value has an owner
/// 2. only one owner at a time
/// 3. owner goes out of the scope - value is dropped

/// 1. fixed size is copied
/// 2. Unknown size is moved
#[test]
fn code1() {
    let x = 5;
    let y = x; // copy

    let s1 = String::from("hello");
    let s2 = s1.clone(); // copy
    let s3 = s1; // move, no more s1
}

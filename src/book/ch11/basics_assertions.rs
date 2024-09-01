fn adder(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test1() {
    /// equality assertion
    assert_eq!(adder(2, 2), 4);
}

#[test]
fn test2() {
    /// non-equality assertion
    assert_ne!(adder(2, 2), 5);
}

#[test]
fn test3() {
    /// boolean assertion
    assert!(adder(2, 2) == 4);
}

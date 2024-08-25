fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn it_adds_two() {
    assert_eq!(22, add_two(20));
}

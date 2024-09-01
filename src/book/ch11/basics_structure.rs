fn add_two(a: i32) -> i32 {
    a + 2
}

/// this test will be included into the scope of `cargo build`
/// therefore, will:
/// - take a time
/// - impact the size of the executable file
#[test]
fn it_adds_two_v1() {
    assert_eq!(4, add_two(2));
}

/// since it is marked with `#[cfg(test)]`
/// this tests will NOT be included into the scope of `cargo build`
/// it will be compiled and run only during `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two_v2() {
        assert_eq!(12, add_two(10));
    }
}

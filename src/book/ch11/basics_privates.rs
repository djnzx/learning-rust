pub fn adder(a: i32, b: i32) -> i32 {
    internal_adder(a, b)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    /// allow to access to non-public
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

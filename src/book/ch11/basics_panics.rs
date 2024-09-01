/// defining a struct
#[derive(Debug, PartialEq)]
struct Guess {
    value: i32,
}

impl Guess {
    /// smart constructor
    /// TODO: is it possible to show in signature, that code can panic?
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[test]
fn test_normal() {
    let x = Guess::new(42);
    let expected = Guess { value: 42 };
    assert_eq!(x, expected)
}

/// checking `panic` fact without checking exact message
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

/// checking `panic` fact WITH exact message match
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")] // message contains
fn greater_than_100_precise() {
    Guess::new(200);
}

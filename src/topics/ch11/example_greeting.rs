fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[test]
fn greeting_contains_1() {
    let g = greeting("Carol");
    let expr = g.contains("Carol") && g.contains("Hello");
    assert!(expr);
}

#[test]
fn greeting_contains_customizable_error_message() {
    let g = greeting("Carol");
    assert!(
        g.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        g
    );
}

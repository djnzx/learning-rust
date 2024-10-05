enum Color {
    Red = 1,
    Green = 3,
    Blue = 5,
}

#[test]
fn code() {
    let x = Color::Red as u32;

    println!("{}", x);
}

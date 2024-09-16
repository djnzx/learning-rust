#[test]
fn split_ok() {
    let s = "Hello!";
    let (s1, s2) = s.split_at(3);

    println!("s1: {}, s2: {}", s1, s2);
}

fn split_not_ok() {
    let s = "Привіт";
    let (s1, s2) = s.split_at(3);

    println!("s1: {}, s2: {}", s1, s2);
}

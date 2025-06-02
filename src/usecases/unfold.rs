#[test]
fn main() {
    // Unfold-like behavior using successors
    let iter = std::iter::successors(Some(0), |&state| if state < 10 { Some(state + 1) } else { None });

    for val in iter {
        println!("{}", val);
    }
}

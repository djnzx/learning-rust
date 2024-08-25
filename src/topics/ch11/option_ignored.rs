/// we can make test `ignored`
/// it will be skipped when we run `cargo test`
///
/// it can be run explicitly via
/// `cargo test -- --ignored`
///
/// it can be included into the scope explicitly
/// `cargo test -- --include-ignored`
#[test]
#[ignore]
fn contains_output() {
    println!(">>>>>>>> println inside test");
    assert_eq!(2 + 2, 4)
}

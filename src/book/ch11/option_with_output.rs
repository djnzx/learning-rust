/// by default `cargo test contains_output` doesn't show the output,
/// but we can include output by specifying `--show-output` option
/// so, `cargo test contains_output -- --show-output` will show it
#[test]
fn contains_output() {
    println!(">>>>>>>> println inside test");
    assert_eq!(2 + 2, 4)
}

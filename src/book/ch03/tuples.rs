#[test]
fn playground() {
    type A = (i32, f64, bool, String);

    /// inferred
    let t = (1, 2.5, true, "hello");
    /// explicit
    let t: (i32, f64, bool, &str) = (1, 2.5, true, "hello");

    /// accessing
    let fst = t.0;

    /// destructurization
    let (q, w, e, r) = t;

    /// empty tuple = unit!
    let t: () = ();
    let s = size_of_val(&t);
    println!("{}", s);
    assert_eq!(s, 0);
}

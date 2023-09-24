fn playground() {

    /// create
    let t: (i32, f64, bool, &str) = (1, 2.5, true, "hello");

    /// accessing
    let fst = t.0;

    /// destructurization
    let (q,w,e,r) = t;

    /// empty tuple
    let t: () = ();
}
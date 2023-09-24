fn playground() {

    let s1 = String::from("hello");
    let s2 = s1;         // move occurs because `s1` has type `String`, which does not implement the `Copy` trait.
    //println!("{}, world!", s1); // value borrowed here after move

    let s3 = s2.clone();
    println!("{}, world!", s2);
    println!("{}, world!", s3);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);            // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                           // x comes into scope

    makes_copy(x);                // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    {
        let mut s: String = String::from("hello");

        let r1 = &mut s;
        // let r2 = &s;     // borrowing
        // let r2 = &mut s; // only one is allowed

        println!("{}, ", r1);
        // println!("{}, {}", r1, r2);
    }

}
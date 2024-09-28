/// if we declare without `&`
/// it will take the ownership
/// and fail to compile in the line 15
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[test]
fn rb1() {
    let s1 = String::from("hello");

    // passing reference gives the way not to lose control
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn change(s: &mut String) {
    s.push_str(", world");
}

#[test]
fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);
    dbg!(s);
}

fn will_not_compile() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    /// will not compile here
    /// let r2 = &mut s;

    dbg!(r1);
}

fn will_compile() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn will_not_compile2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // We also cannot have a mutable reference while we have an immutable one to the same value.
    /// will not compile here
    /// let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);
}

fn will_compile_2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
//
// fn main_dangling() {
//     let reference_to_nothing = dangle();
// }

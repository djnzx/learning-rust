/// heap
/// https://doc.rust-lang.org/std/collections/index.html
/// https://doc.rust-lang.org/book/ch08-01-vectors.html
fn playground1() {
    /// generic Vec<T>

    /// new empty
    let v: Vec<i32> = Vec::new();
    /// macros to put elements
    let v = vec![1, 2, 3];
    // new mutable
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /// unsafe, can panic in runtime, copy value
    let third: i32 = v[2]; // 7
    /// unsafe, can panic in runtime, reference to value
    let third_p: &i32 = &v[2]; // 7
    /// safe, option can be matched, reference
    let third: Option<&i32> = v.get(2);

    match third {
        Some(t) => println!("The third element is {t}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    let idx = 100;
    // panic
    // let does_not_exist = &v[100];
    // no panic
    let does_not_exist = v.get(idx);

    match does_not_exist {
        None => { println!("{}-th <none>", idx) }
        Some(x) => { println!("{}-th, some<{}>", idx, x) }
    }

}

pub fn playground2() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
    // v.push(6);            // mutable borrow occurs here, modification after borrow. WILL NOT COMPILE
    println!("The first element is: {first}"); //immutable borrow later used here
}

/// iteration
pub fn playground12() {
    /// we can copy values
    let v: Vec<i32> = vec![10, 20, 30];
    for i in v {
        println!("{i}");
    }
    println!("--------");

    /// we can copy get a reference
    let v: Vec<i32> = vec![10, 20, 30];
    for i in &v {
        println!("{i}");
    }
    println!("========");

    /// we can mutate
    let mut v = vec![10, 32, 57];
    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 10;
    }
    println!("========");

    v.iter().for_each(|x|println!("{x}"));

    /// enum for element unification
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

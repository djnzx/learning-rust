/// heap
/// https://doc.rust-lang.org/std/collections/index.html
fn playground1() {
    /// new empty
    /// generic Vec<T>
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /// unsafe, copy
    let third: i32 = v[2]; // 7
    /// unsafe, reference
    let third_p: &i32 = &v[2]; // 7

    /// safe, reference
    let third: Option<&i32> = v.get(2);
}

pub fn playground2() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
    // v.push(6); // mutable borrow occurs here
    println!("The first element is: {first}"); //immutable borrow later used here
}

/// iteration
pub fn playground12() {
    let v: Vec<i32> = vec![100, 32, 57];
    /// we can copy values or get a reference
    for i in &v {
        println!("{i}");
    }
    println!("--------");
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    println!("--------");
    for i in &mut v {
        *i += 50;
    }
    v.iter().for_each(|x|println!("{x}"));

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

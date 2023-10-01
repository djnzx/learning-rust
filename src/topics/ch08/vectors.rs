#[test]
fn test1() {
    // create empty
    let v: Vec<i32> = Vec::new();

    // macro to build non-empty
    let v = vec![1, 2, 3];
    dbg!(v);

    // mutable
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    dbg!(v);
}

#[test]
fn accessing() {
    let v = vec![1, 2, 3, 4, 5];

    // unsafe access
    let third: &i32 = &v[2]; // 3
    println!("The third element is {third}");

    // safe access
    let third: Option<&i32> = v.get(2); // Some(3)
    let sixth: Option<&i32> = v.get(6); // None

    dbg!(third);
    dbg!(sixth);

    // slices
    let slice1 = &v[..3]; // not including: 1,2,3
    let slice2 = &v[2..]; // including:     3,4,5

    dbg!(slice1);
    dbg!(slice2);
}

#[test]
fn iteration() {
    let v = vec![10, 20, 30, 40, 50];
    for i in &v {
        println!("{i}");
    }
}

#[test]
fn modification() {
    let mut v = vec![10, 20, 30, 40, 50];
    for i in &mut v {
        *i += 1;
    }
    dbg!(v);
}

#[test]
fn custom_type() {
    #[derive(Debug)]
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

    row.iter().for_each(|x| {
        dbg!(x);
        ()
    });

    row.iter().enumerate().for_each(|x| {
        dbg!(x);
        ()
    });
}

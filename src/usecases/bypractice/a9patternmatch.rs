#[test]
fn code1() {
    let x = Some(15);

    /// full handle types must match
    let y = match x {
        None => 0,
        Some(k) => k,
    };

    /// partial handing of match statement, therefore can't return
    /// actually used for cases when to run something
    if let Some(y) = x {
        println!("{}", y);
    }
}

#[test]
fn code2() {
    let alpha = ['a', 'E', 'Z', '0', 'x', '9', 'y'];
    for ab in alpha {
        /// macro to generate match statements
        let m = matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9');
        assert!(m);
    }
}

#[test]
fn code3() {
    let x = 3;
    let z = match x {
        1 => "One",
        2 | 3 => "Two or Three",
        4..10 => "4 till 10",
        _ => "10 or more",
    };
}

struct Point(i32, i32);

/// ranges and OR supported, naming as in scala or haskell
fn literal_match(p: &Point) -> String {
    match p {
        Point(0, 0) => "center".to_owned(),
        Point(x, 0) => format!("x = {}, y = 0", x),
        Point(0, y) => format!("x = 0, y = {}", y),
        p @ Point(x, y) => format!("x = {}, y = {}", x, y),
    }
}

#[test]
fn code5() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (fst, snd, .., pen, lst) => {
            assert_eq!(fst, 2);
            assert_eq!(snd, 4);
            assert_eq!(pen, 1024);
            assert_eq!(lst, 2048);
        }
    }
}

#[test]
fn code6() {
    let mut v = String::from("hello");
    let r = &mut v;
    match r {
        v => v.push_str(", world"),
    }
    println!("{}", v);
}

#[test]
fn code7() {}

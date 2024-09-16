#[test]
fn code1() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn compare(x: Option<u8>) -> String {
    match x {
        Some(50) => "Got 50".to_string(),
        Some(16) | Some(32) => "Got 2^x".to_string(),
        Some(99..=101) => "Range 99-101 matched".to_string(), // chars work as well
        Some(x) => format!("Matched Some({x}), but not Some(50)"),
        _ => format!("Default case, x = {x:?}"),
    }
}

#[test]
fn code2() {
    let s = compare(Some(50));
    println!("{}", s);

    let s = compare(Some(16));
    println!("{}", s);

    let s = compare(Some(20));
    println!("{}", s);

    let s = compare(Some(99));
    println!("{}", s);

    let s = compare(None);
    println!("{}", s);
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn code3() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

#[test]
fn code4() {
    let p = Point { x: 0, y: 7 };

    // names must match!
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn code5(p: Point) {
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn code6(msg: Message) {
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}")
        }
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn nested_as_well(msg: Message2) {
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
#[test]
fn any_combination() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn we_can_ignore_parameter(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}
fn ignore_tail(p: Point3d) {
    match p {
        Point3d { x, .. } => todo!(),
    }
}

fn ignore_whatever(p: (u8, u8, u8, u8, u8, u8)) {
    match p {
        (x, _, _, _, _, _) => todo!(),
        (x, .., last) => todo!(),
        (.., last) => todo!(),
    }
}
fn extra(x: u8) {
    match x {
        5 => todo!(),
        x if x % 2 == 0 => todo!(),
        x => todo!(),
    }
}

enum Message3 {
    Hello { id: i32 },
}

fn binding(m: Message3) {
    match m {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}

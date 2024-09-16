#[test]
fn code1() {
    let x = 7;
    let y = 5;
    let z = x / y;
    let k = x % y;

    assert_eq!(z, 1);
    assert_eq!(k, 2);
}

#[test]
fn code2() {
    let x = -7;
    let y = 3;
    let z = x / y;
    let k = x % y;

    assert_eq!(z, -2);
    assert_eq!(k, -1);
}

fn euclidean_mod(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r < 0 {
        r + b
    } else {
        r
    }
}

#[test]
fn code4() {
    let x = -7;
    let y = 3;
    let z = x / y;
    let k = euclidean_mod(x, y);

    assert_eq!(z, -2);
    assert_eq!(k, 2);
}

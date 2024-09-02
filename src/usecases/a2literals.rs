#[test]
fn literals() {
    let x = 1_024 + 0xff + 0077 + 0b1111_1111;
}

#[test]
fn extend_narrow() {
    let x = 1234u16;
    let y = x as u32;
    let z = x as u8;

    assert_eq!(210, z); // 1234-256-256-256-256

    println!("{} {} {}", x, y, z);
}

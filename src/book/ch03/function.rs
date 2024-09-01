/// literals allowed
/// type is the must
fn add1(x: u32, y: u32) -> u32 {
    x + y
}

fn add2(x: &u32, y: &u32) -> u32 {
    x + y
}

#[test]
pub fn sandbox() {
    let z1 = add1(1, 2);
    let z2 = add2(&1, &2);
    dbg!(z1);
    dbg!(z2);
    // unit ;)
    let x = {
        let y = 6;
    };
}

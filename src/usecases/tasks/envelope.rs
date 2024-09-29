#[test]
fn envelope() {
    const W: u32 = 30;
    const H: u32 = 10;
    let k = W as f32 / H as f32;

    let is_diag1 = |x: u32, y: u32| ((y as f32) * k).floor() as u32 == x;
    let is_diag2 = |x: u32, y: u32| ((y as f32) * k).floor() as u32 == W - x + 1;

    for y in 1..=H {
        for x in 1..=W {
            let c = match (y, x) {
                (1, _) => "*",
                (H, _) => "*",
                (_, 1) => "*",
                (_, W) => "*",
                (y, x) if is_diag1(x, y) => "*",
                (y, x) if is_diag2(x, y) => "*",
                (_, _) => " ",
            };
            print!("{}", c);
        }
        println!()
    }
}

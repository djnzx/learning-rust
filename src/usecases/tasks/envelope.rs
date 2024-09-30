#[test]
fn envelope() {
    const W: u32 = 30;
    const H: u32 = 15;
    let k = W as f32 / H as f32;

    fn mul(a: u32, k: f32) -> u32 {
        (a as f32 * k).ceil() as u32
    }

    for y in 1..=H {
        for x in 1..=W {
            let is_diag = mul(y, k) == x;
            let is_co_diag = mul(y, k) == W - x + 1;

            let c = match (y, x) {
                (1 | H, _) => "*",
                (_, 1 | W) => "*",
                _ if is_diag || is_co_diag => "*",
                _ => " ",
            };
            print!("{}", c);
        }
        println!()
    }
}

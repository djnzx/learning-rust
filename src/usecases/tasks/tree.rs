#[test]
fn test1() {
    const W: u32 = 11;
    const H: u32 = 6;
    const N: u32 = 3;

    let mirror = |x: u32| W - 1 - x;

    for n in 0..N {
        for y in 0..H {
            for x in 0..W {
                let q1 = x + y < W / 2;
                let q2 = mirror(x) + y < W / 2;
                let c = if q1 || q2 { ' ' } else { '*' };
                print!("{c}");
            }
            println!();
        }
    }
}

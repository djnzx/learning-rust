#[test]
fn test1() {
    const SIZE: u32 = 11;
    let half = SIZE / 2;
    let inv = |x: u32| SIZE - 1 - x;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let q1 = x + y < half;
            let q2 = inv(x) + y < half;
            let q3 = inv(x) + inv(y) < half;
            let q4 = x + inv(y) < half;
            let c = if q1 || q2 || q3 || q4 { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}

#[test]
fn test2() {
    const SIZE: u32 = 11;
    let half = SIZE / 2;
    let inv = |x: u32| SIZE - 1 - x;
    for j in 0..SIZE * SIZE {
        let y = j / SIZE;
        let x = j % SIZE;
        let q1 = x + y < half;
        let q2 = inv(x) + y < half;
        let q3 = inv(x) + inv(y) < half;
        let q4 = x + inv(y) < half;
        let c = if q1 || q2 || q3 || q4 { ' ' } else { '*' };
        print!("{c}");
        if x == SIZE - 1 {
            println!();
        }
    }
}

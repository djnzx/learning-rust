#[test]
fn test_1_float_example() {
    let x = 0.3125_f32;
    println!("{:0>32b}", x.to_bits());
    // 00111110101000000000000000000000
    // ^\______/\_____________________/
    // |   |        |
    // |   |        -- T
    // |   ----------- E
    // --------------- sign
}

#[test]
fn test_2_float_to_binary() {
    let x = 1.0_f32;
    // IEE 754 https://uk.wikipedia.org/wiki/IEEE_754
    let xs = x.to_bits();
    println!("{:b}", xs);

    let x = 1.1_f32;
    let xs = x.to_bits();
    println!("{:b}", xs);
}

#[test]
fn test_3_float_is_non_precise_1() {
    //          123456790000000000
    let x = 123456789123456789_f32;
    println!("{}", x);
    println!("{:e}", x);
}

#[test]
fn test_3_float_is_non_precise_2() {
    let x = 16777216_f32;
    println!("{}", x);
    let x = 16777217_f32;
    println!("{}", x);
    let x = 1 << 24;
    println!("{}", x);
}

#[test]
fn test_3_float_is_non_precise_3() {
    let x = 1_f32 / 3_f32;
    println!("{}", x);

    let x = 0.2 + 0.1;
    println!("{}", x);
}

#[test]
fn test_3_float_sometimes_precise() {
    let x = 0_f32;
    println!("{:0>32b}", x.to_bits());

    let x = 1_f32;
    println!("{:0>32b}", x.to_bits());

    let x = 1_f32 / 2_f32;
    println!("{:0>32b}", x.to_bits());

    let base = 2.0f32;
    let n = base.powi(-127);

    println!("{0:e} {0} {1:0>32b}", n, n.to_bits());
}

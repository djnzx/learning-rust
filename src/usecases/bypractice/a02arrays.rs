#[test]
fn code1() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(5, a.len());
    let a1 = a[1];
    let maybe_a10 = a.get(10);
}

#[test]
fn code2() {
    let a = ['a', 'b'];
    assert_eq!(2 * 4, size_of_val(&a));
}

#[test]
fn code3() {
    //               master
    let ls = [1; 11];
    println!("{:?}", ls);
}

#[test]
fn code4() {
    // slices can be taken from any things
    // implementing Deref trait
    let a = [1, 2, 3, 5, 6, 7, 8, 9];
    let s: &[i32] = &a[0..5];
    /// size of array
    assert_eq!(a.len() * std::mem::size_of::<i32>(), size_of_val(&a)); // 8 * 4
    /// size of slice
    assert_eq!(s.len() * std::mem::size_of::<i32>(), size_of_val(s)); // 5 * 4
    /// size of slice pointer
    assert_eq!(2 * std::mem::size_of::<usize>(), size_of_val(&s)); // 2 * 8
}

#[test]
fn code5() {
    let ls = ['a', 'b', 'c', 'd'];

    for (idx, val) in ls.iter().enumerate() {
        println!("idx: {}, val: {}", idx, val);
    }
}

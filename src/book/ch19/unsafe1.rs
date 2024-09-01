#[test]
fn code1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // *r2 = 6;
    unsafe {
        *r2 = 6;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

#[test]
fn code2() {
    // random address
    let address = 0x012345usize;
    // dereference as value of type i32
    let r = address as *const i32;

    println!("{:?}", r)
}

#[test]
fn code3() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

#[test]
fn code4() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

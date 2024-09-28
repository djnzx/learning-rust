use std::fmt::Debug;

// https://www.youtube.com/watch?v=BpPEoZW5IiY&t=8650s
#[test]
fn test1() {
    // struct will alive till `x` is alive
    struct A<'a> {
        x: &'a u32,
        y: u32,
    }
}

#[test]
fn test2() {
    let x = 5;
    let px = &x;
    println!("{:p}", px);
    assert_eq!(5, *px);
}

fn borrowing(s: &String) {}

#[test]
fn test3a() {
    let s = String::from("hello");
    borrowing(&s);
    // s still alive
    println!("{}", s);
}

// 1. to make push we need to declare `mut`
fn borrowing_and_mutate(s: &mut String) {
    s.push_str("!")
}

#[test]
fn test3b() {
    // 3. as we want to mutate we need to declare it `mut`
    let mut s = String::from("hello");
    // 2. since we expect mut, we need to say `mut` here
    borrowing_and_mutate(&mut s);
    // s still alive
    println!("{}", s);
}

#[test]
fn test4() {
    let mut s: String = String::from("hello, ");
    let p = &mut s;
    // actually automatic dereference here (*p)
    p.push_str("world");
    println!("{}", p);
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
fn test5() {
    let c: char = 'z';
    // syntax #1
    let r1 = &c;
    // syntax #2
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    println!("{:?}", get_addr(r1));
    println!("{:?}", get_addr(r2));
}

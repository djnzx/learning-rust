// dependent types
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct S1 {}
struct S2 {}
trait T1 {
    fn go(&self);
}
impl T1 for S1 {
    fn go(&self) {
        println!("go from S1")
    }
}
impl T1 for S2 {
    fn go(&self) {
        println!("go from S2")
    }
}
#[test]
fn code1() {
    let s1: S1 = S1 {};
    let s2: S2 = S2 {};

    s1.go();
    s2.go();
}

trait T10 {
    type A;
    fn make() -> Self::A;
}

impl T10 for S1 {
    type A = u8;

    fn make() -> Self::A {
        1u8
    }
}
impl T10 for S2 {
    type A = u16;

    fn make() -> Self::A {
        2u16
    }
}

#[test]
fn code2() {
    // instances creation
    let s1 = S1 {};
    let s2 = S2 {};

    // dynamic call
    s1.go();
    s2.go();

    // static methods
    let x1 = S1::make();
    let x2 = S2::make();
}

use std::ops::Add;

struct A;
struct S(A);
struct Box<A>(A);

fn regular(_s: S) {}
fn gen_spec_i32(_s: Box<i32>) {}
fn gen_spec_a(_s: Box<A>) {}
fn gen_spec<A>(_s: Box<A>) {}

/// struct without fields has only ONE value
/// enum without values has NO values
#[test]
fn code1() {
    regular(S(A));
    gen_spec_i32(Box(13));
    gen_spec_a(Box(A));
    //         --------->
    gen_spec::<f64>(Box(3.14));
    //       <-----
    gen_spec(Box(3.14));
}

fn sum<A: Add<Output = A>>(a: A, b: A) -> A {
    a + b
}

#[test]
fn code2() {
    let a = sum(1u8, 2u8);
    let b = sum(1f32, 2f32);
}

struct Point<A> {
    x: A,
    y: A,
}

impl<A> Point<A> {
    fn values(&self) -> (&A, &A) {
        (&self.x, &self.y)
    }
}

#[test]
fn code3() {
    let p1: Point<i32> = Point { x: 1, y: 2 };
    let p2: Point<f64> = Point { x: 1.3, y: 2.4 };
}

struct Point2<A, B> {
    x: A,
    y: B,
}

impl<A, B> Point2<A, B> {
    // we write self, not &self since we take ownership
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<A, W> {
        Point2 { x: self.x, y: other.y }
    }
}

#[test]
fn code4() {
    let p: Point2<i32, f64> = Point2 { x: 1, y: 2.3 };
}

struct Array<A, const N: usize> {
    data: [A; N],
}

#[test]
fn code5() {
    let xs: [Array<i32, 2>; 3] = [
        Array { data: [1, 2] }, // all must have the same size
        Array { data: [2, 3] },
        Array { data: [4, 5] },
    ];
}

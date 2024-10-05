use std::any::Any;

trait Printable {
    type TT;

    fn get(&self) -> Self::TT;
}

trait PrintableG<TT> {
    fn get(&self) -> TT;
}

struct Box1 {}
struct Rect {}

impl Printable for Box1 {
    type TT = i32;

    fn get(&self) -> Self::TT {
        16
    }
}

impl PrintableG<i32> for Box1 {
    fn get(&self) -> i32 {
        17
    }
}

impl Printable for Rect {
    type TT = f32;

    fn get(&self) -> Self::TT {
        32.0
    }
}

impl PrintableG<f32> for Rect {
    fn get(&self) -> f32 {
        33.0
    }
}

/// we have 2 different syntax
/// to allow distinguishing situation where
/// Printable<TT=i32> and Printable<TT=u32> are the same
/// but PrintableG<i32> and PrintableG<u32> are different
///
/// when we don't want to expose internal details
///
fn go1<A: Printable>(a: &A) {
    let x: <A as Printable>::TT = a.get();
}

fn go2<A: PrintableG<B>, B>(a: &A) -> B {
    let x: B = a.get();
    x
}

#[test]
fn code1() {
    let x = go1(&Box1 {});
    let x = go1(&Rect {});
    let x = go2(&Box1 {});
    let x = go2(&Rect {});
}

trait Bird: Any {
    fn quack(&self) -> String;
    // this allows instance to be represented as Any
    fn as_any(&self) -> &dyn Any;
}

struct Duck;
struct Swan;

impl Duck {
    fn swim(&self) {
        println!("Duck is swimming");
    }
}

impl Swan {
    fn fly(&self) {
        println!("Swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        String::from("Duck is a bird")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        String::from("Swan is a bird")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn hatch_a_bird(n: u8) -> Box<dyn Bird> {
    match n {
        1 => Box::new(Duck),
        2 => Box::new(Swan),
        _ => todo!(),
    }
}

#[test]
fn code2() {
    let duck: Duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "Duck is a bird");

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "Swan is a bird");
}

#[test]
fn code3() {
    let birds: [&dyn Bird; 2] = [&Swan, &Duck];
    for b in birds.iter() {
        println!("{}", b.quack());
        let x = b.type_id();
        println!("{:?}", x);
        let a: &dyn Any = b.as_any();
        let b1 = a.downcast_ref::<Duck>();
        if let Some(duck) = b.as_any().downcast_ref::<Duck>() {
            duck.swim();
        } else if let Some(swan) = b.as_any().downcast_ref::<Swan>() {
            swan.fly();
        }
    }
}

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for i32 {
    fn draw(&self) -> String {
        format!("i32:{}", self)
    }
}
impl Draw for f32 {
    fn draw(&self) -> String {
        format!("f32:{}", self)
    }
}

fn draw_boxed<A: Draw>(a: &Box<A>) {
    a.draw();
}

fn draw_ref(a: &dyn Draw) {
    a.draw();
}

#[test]
fn code4() {
    let x = 1.234;
    let y = 123;

    draw_boxed(&Box::new(x));
    draw_boxed(&Box::new(y));

    draw_ref(&x);
    draw_ref(&y);

    assert_eq!(x.draw(), "f32:1.234");
    assert_eq!(y.draw(), "i32:123");
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8:{}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string:{}", *self)
    }
}

/// fn static_dispatch<u8>(a: u8)
/// in compile time
fn static_dispatch<A: Foo>(a: A) {
    let x = a.method();
}

/// runtime resolution via vtable
/// that's why pointer
fn dynamic_dispatch(a: &dyn Foo) {
    let x = a.method();
}

#[test]
fn code5() {
    let x = 5u8;
    let y = "rust".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);
}

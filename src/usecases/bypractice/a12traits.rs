use std::ops;
use std::ops::Add;

/// https://www.youtube.com/watch?v=BpPEoZW5IiY&t=22000s
/// paused @ 7:02:11
/// static vs dynamic dispatch

trait Animal {
    fn sound(&self) -> String;
}

/// Display {}
/// Debug {:?}
/// Clone -with call to .clone()
/// Copy - without call to .clone()
/// PartialEq - for comparing
/// * === Add trait
///
struct Sheep;
struct Cow;

impl Animal for Sheep {
    fn sound(&self) -> String {
        String::from("maah")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("mooh")
    }
}

/// different syntax allowed
/// many traits:
/// A : Display + Clone

fn notify1(item: &impl Animal) {
    let x = item.sound();
}

fn notify2<A: Animal>(item: &A) {
    let x = item.sound();
}

fn notify3<A>(item: &A)
where
    A: Animal,
{
    let x = item.sound();
}

// invariant. can't return Sheep/Cow in one method
fn mk_sheep() -> impl Animal {
    Sheep {}
}

fn mk_cow() -> impl Animal {
    Cow {}
}

/// trait objects, allows subtyping, since size of pointer is known
/// requires lifetime, actually forever, so not good
fn mk_random1(value: f64) -> &'static dyn Animal {
    if value < 0.5 {
        &Sheep
    } else {
        &Cow
    }
}

/// allows subtyping, since size of pointer is known
/// owns, can be cloned, can be used in pattern matching
fn mk_random2(value: f64) -> Box<dyn Animal> {
    if value < 0.5 {
        Box::new(Sheep)
    } else {
        Box::new(Cow)
    }
}

#[test]
fn code1() {
    let s = Sheep {};
    let c = Cow {};

    println!("{}", s.sound());
    println!("{}", c.sound());
    assert_eq!("maah", s.sound());
    assert_eq!("mooh", c.sound());
}

struct Inches(i32);
struct Centimeters(f64);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

impl Centimeters {
    fn to_inches(&self) -> Inches {
        let &Centimeters(cm) = self;
        Inches((cm as f64 / 2.54) as i32)
    }
}

fn mult<A: ops::Mul<Output = A>>(a1: A, a2: A) -> A {
    a1 * a2
}

#[test]
fn code2() {
    let a = mult(1u8, 2u8);
    let a = mult(1.8, 2.8);
}

struct Electro;
struct Car;

/// we need PartialEq co compare inside assert
#[derive(PartialEq, Debug)]
struct Tesla;

impl Add<Electro> for Car {
    type Output = Tesla;

    fn add(self, rhs: Electro) -> Self::Output {
        Tesla
    }
}

#[test]
fn code3() {
    assert_eq!(Tesla, Car + Electro);
}

#[test]
fn code4() {}

#[test]
fn code5() {}

#[test]
fn code6() {}

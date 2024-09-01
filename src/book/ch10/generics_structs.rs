struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn main1() {
    let integer: Point<i32> = Point { x: 5, y: 10 };
    let float: Point<f64> = Point { x: 1.0, y: 4.0 };
}

struct Point2<T, U> {
    x: T,
    y: U,
}

#[test]
fn main2() {
    let both_integer: Point2<i32, i32> = Point2 { x: 5, y: 10 };
    let both_float: Point2<f64, f64> = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float: Point2<i32, f64> = Point2 { x: 5, y: 4.0 };
    let another_point: Point2<&str, char> = Point2 { x: "5", y: '4' };
}

enum Option1<T> {
    Some1(T),
    None1,
}

enum Result1<T, E> {
    Ok1(T),
    Err1(E),
}

impl<T> Point<T> {
    /// getter method
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

#[test]
fn main3() {
    let p1: Point2<i32, f64> = Point2 { x: 5, y: 10.4 };
    let p2: Point2<&str, char> = Point2 { x: "Hello", y: 'c' };

    let p3: Point2<i32, char> = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

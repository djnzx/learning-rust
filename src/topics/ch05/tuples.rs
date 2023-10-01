/// structs can be
/// without names
/// they are like tuples, but have name
#[derive(Debug)]
struct Color(i32, i32, i32); // r,g,b
#[derive(Debug)]
struct Point(i32, i32, i32); // x,y,z
#[derive(Debug, PartialEq)]
struct AlwaysEqual; // named, typed, unit, singleton

// can be derived
impl PartialEq for Color {
    // Self means the same type (Color)
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[test]
fn creation() {
    let black: Color = Color(1, 2, 3);
    let origin: Point = Point(4, 5, 6);
    let subject = AlwaysEqual;
    dbg!(&black, origin, subject);
}
#[test]
fn equality() {
    let black: Color = Color(1, 2, 3);
    let eq = black == Color(1, 2, 3);
    dbg!(eq);

    // two objects will be created, look for lazy_static! or OnceCell
    let x = AlwaysEqual;
    let y = AlwaysEqual;
    // need to derive Clone
    // let z = x.clone();

    dbg!(x == y); // true
    dbg!(std::ptr::eq(&x, &y)); // false
}

#[test]
fn unit_type() {
    // empty tuple
    let unit = ();
    dbg!(unit)
}

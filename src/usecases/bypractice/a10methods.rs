use rand::Rng;

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// we can have multiple impl blocks
impl Rectangle {
    /// static methods (without self), associated functions
    //                                 Self can be used instead of Rectangle
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    fn random() -> Rectangle {
        let mut rt = rand::thread_rng();
        let width = rt.gen_range(5..50);
        let height = rt.gen_range(5..50);
        Rectangle::new(width, height)
    }
    /// instance function (with self) methods
    fn scale(&self, k: f32) -> Rectangle {
        Rectangle {
            width: (self.width as f32 * k) as u32,
            height: (self.height as f32 * k) as u32,
        }
    }
}

#[test]
fn code1() {
    let r1 = Rectangle { width: 10, height: 20 };
    let r2 = r1.scale(1.5);

    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{}", r1 == r2); // PartialEq required
}

#[test]
fn code2() {
    let r1 = Rectangle::random();

    println!("{:?}", r1);
}

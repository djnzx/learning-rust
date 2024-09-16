// naive implementation
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// more semantics
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// way better
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// attaching methods
impl Rectangle {
    /// with context
    /// &self means - is not static
    fn area(&self) -> u32 {
        self.height * self.width
    }
    // all functions are static, we can use them for free from any place
    fn area2(&self) -> u32 {
        area3(self)
    }

    /// to make it working, it should be declared
    /// let mut r = Rectangle {...}
    fn scale(&mut self, k: u32) {
        self.height *= k;
        self.width *= k;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn can_hold2(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    // we can have methods with the same names as fields
    // field:  r.width
    // method: r.width()
    fn width(&self) -> bool {
        self.width > 0
    }
}

// we can have many impl blocks
// so, actually they are extension methods
impl Rectangle {
    /// w/o context
    /// absence of &self - means IS STATIC
    /// Self - is the type in the impl block (Rectangle)
    fn square(size: u32) -> Self {
        Self {
            width: dbg!(size), // dbg! - returns
            height: size,
        }
    }
}

#[test]
fn playground2() {
    let mut r = Rectangle { width: 10, height: 10 };

    println!("The area of the rectangle is {} square pixels.", r.area());
    let sq = Rectangle::square(3);
    dbg!(&r);

    r.scale(2);
    dbg!(&r);
}

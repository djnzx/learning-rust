type Kilometers = i32;

fn go(x: Kilometers) {}

#[test]
fn code1() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    // aliases don't give safety
    go(33);
}

// but sometimes readability give
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

// type Nothing
fn bar() -> ! {
    panic!();
}

fn generic1<T>(t: T) {
    // --snip--
}

fn generic2<T: Sized>(t: T) {
    // --snip--
}

fn generic3<T: ?Sized>(t: &T) {
    // --snip--
}
#[test]
fn code2() {}

#[test]
fn code3() {}

#[test]
fn code4() {}

#[test]
fn code5() {}

#[test]
fn code6() {}

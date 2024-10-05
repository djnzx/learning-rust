trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self {
        42
    }
}

impl MyTrait for String {
    fn f(&self) -> Self {
        String::from("todo!")
    }
}

// static dispatch
// monomorphisation is done in compile time
fn my_fn1<A: MyTrait>(x: Box<A>) -> A {
    x.f()
}

#[test]
fn code6() {
    let x = my_fn1(Box::new(42u32));
    let x = my_fn1(Box::new(String::from("rust")));
}

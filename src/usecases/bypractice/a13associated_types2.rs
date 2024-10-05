trait MyTrait2 {
    fn f(&self) -> Box<dyn MyTrait2>;
}

impl MyTrait2 for u32 {
    fn f(&self) -> Box<dyn MyTrait2> {
        Box::new(42u32)
    }
}

impl MyTrait2 for String {
    fn f(&self) -> Box<dyn MyTrait2> {
        Box::new(String::from("todo!"))
    }
}

// dynamic dispatch
// resolution on runtime
fn my_fn2(x: Box<dyn MyTrait2>) -> Box<dyn MyTrait2> {
    x.f()
}

#[test]
fn code7() {
    let x = my_fn2(Box::new(42u32));
    let x = my_fn2(Box::new(String::from("rust")));
}

use std::ops::Deref;

/// smart pointers implement traits:
/// Deref
/// Drop - explicit destructor
///
/// Box<A> - just to put in heap
/// Rc<A> - to share immutable data
/// Ref<A>
/// RefMut<A>
/// RefCell<A>
///
#[test]
fn code1() {
    // Box - size is not known in compile time
    let b = Box::new(5);
    println!("b={b}");
}

#[test]
fn code2() {
    // will not work since infinite size
    // we need to use box
    // enum List<A> {
    //     Cons(A, List<A>),
    //     Nil,
    // }
    enum List<A> {
        Cons(A, Box<List<A>>),
        Nil,
    }

    use List::Cons;
    use List::Nil;

    let xs: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}

#[test]
fn code3() {
    let x = 5;
    let y = &x; // make a ref
    let z = *y; // deref

    // will not compile
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    println!("{}", x);
    println!("{}", y); // automatic deref
    println!("{}", *y);

    let k = Box::new(5);
    assert_eq!(5, *k); // Box is a ref, an need to be deref
}

#[test]
fn code4() {
    struct MyBox<A>(A); // A has index 0

    impl<A> MyBox<A> {
        fn new(x: A) -> MyBox<A> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    use std::ops::Deref;

    // what is the difference between Deref<A> and Deref type Target = A
    impl<T> Deref for MyBox<T> {
        type Target = T;

        // actually just accessor to allow deref syntax
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let k = y.deref();
    let l = *(y.deref());

    // will not compile, until Deref is implemented
    let z = *y;

    assert_eq!(5, x);
    // assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // without deref we should write
    hello(&(m.0));
    // with deref we can write
    hello(&m);
    // the full syntax, rust will apply
    hello(&((*m)[..]));
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // explicit destructor
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn drop_showcase() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

#[test]
fn manual_early_release() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // not allowed
    // c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    // can't do since there is no c anymore
    // dbg!(c);
}

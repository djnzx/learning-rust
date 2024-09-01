use std::cell::RefCell;
use std::rc::Rc;

// multiple owners to the mutable data
// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

/// RefCell<T> does not work for multithreaded code!
#[test]
fn code3() {
    let tail = Rc::new(RefCell::new(5)); // RefCell(5)

    let a = Rc::new(Cons(Rc::clone(&tail), Rc::new(Nil))); // Cons(RefCell(5), Nil)

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); // Cons(RefCell(3), Cons(RefCell(5), Nil))
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)); // Cons(RefCell(4), Cons(RefCell(5), Nil))

    *tail.borrow_mut() += 10;

    println!("a after = {a:?}"); // Cons(RefCell { value: 15 }, Nil)
    println!("b after = {b:?}"); // Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {c:?}"); // Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}

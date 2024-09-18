use std::fmt::{Debug, Formatter};

#[test]
fn test1() {
    let x: String = String::from("hello");
    //              without clone, it is moved
    let y: String = x.clone();
    println!("{} {}", x, y);
}

#[test]
fn test1a() {
    let x: String = String::from("hello");
    let y = &x;
    println!("{} {}", x, y);
}

#[test]
fn test2() {
    let s: String = give();
    println!("{}", s);
}

fn give() -> String {
    let s: String = String::from("hello");
    // consumes and makes `s` unavailable
    // let _s1: Vec<u8> = s.into_bytes();
    // doesn't consume, therefore `s` is still available
    let _s: &[u8] = s.as_bytes();
    s
}

#[test]
fn test3a() {
    let x = (1, (), "hello".to_string());
    let y = x.clone();

    println!("{:?} {:?}", x, y)
}

#[test]
fn test3b() {
    //                          V - immutable, known at compile time
    let x = (1, (), "hello");
    // Copy done implicitly, since everything in the stack
    let y = x;

    println!("{:?} {:?}", x, y)
}

#[test]
fn test4() {
    // Box is a memory address
    let x = Box::new(5);

    let mut y = Box::new(None::<i32>);
    *y = Some(4); // explicit unbox

    assert_eq!(*x, 5);
    assert_eq!(*y, Some(4));
}

#[test]
fn test5() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Jim"),
        age: Box::new(20),
    };

    //   without ref the `name` will be moved
    let Person { ref name, ref age } = person;

    println!("{}", name);
    println!("{}", age);
    println!("{:?}", person); // use DDebug trait
}

#[test]
fn test6_partial_move() {
    let t = (String::from("a"), String::from("b"));
    let s1 = t.0;
    // no way to access `t` anymore
    println!("{:?}", t.1);
}

#[test]
fn test7() {
    let t = (String::from("a"), String::from("b"));

    // pulling references doesn't change the ownership
    let (ref s1r, ref s2r) = t;
    let (s1, s2) = t.clone();

    println!("{}{}{:?}", s1, s2, t);
}

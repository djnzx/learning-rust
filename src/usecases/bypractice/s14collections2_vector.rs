use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// String = Vec<u8>
#[test]
fn code1() {
    let a: [u8; 3] = [1, 2, 3];
    // converts
    let b = Vec::from(a);
    let c = vec![1u8, 2, 3];
    // doesn't convert, takes as is
    let d = vec![a]; // vector of one element
                     // todo is there a way to provide the type on the right side ???
    let mut e: Vec<u8> = Vec::new();
    e.push(11);
    let f: Vec<u8> = Vec::with_capacity(10);

    // value borrowed, not moved
    for x in &c {}
    println!("{:?}", c);
    // value moved, not borrowed
    for x in c {}
    // will not compile
    // println!("{:?}", c);
}

#[test]
fn code2() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2];
    let v3 = vec![1, 2, 3];
    println!("{}", v1 == v2);
    println!("{}", v1 == v3);
    assert_eq!(v1 == v2, false);
    assert_eq!(v1 == v3, true);
}

#[test]
fn code3() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    let maybeFive = v.pop();
    println!("{:?}", v);
    v.push(55);
    println!("{:?}", v);
    v[4] = 66;
    println!("{:?}", v);
    let mut v0: Vec<i32> = Vec::new();
    v0.extend(&v); // makes a copy
    println!("{:?}", v0);
    println!("{:?}", v0 == v);
    v0.push(77);
    println!("{:?}", v);
    println!("{:?}", v0);
}

#[test]
fn array_to_vector() {
    let a = [1, 2, 3];
    let v1 = Vec::from(a);
    let v2 = a.to_vec();
    let v3: Vec<i32> = a.into(); // Into trait
    assert_eq!(v1, v2);
    assert_eq!(v1, v3);
}

#[test]
fn string_to_vector() {
    let s1 = String::from("hello world");
    let v1: Vec<u8> = s1.into(); // Into trait

    let s2 = "hello world".to_string();
    let v2 = s2.into_bytes();

    let s3 = "hello world";
    let v3 = Vec::from(s3);

    assert_eq!(v1, v2);
    assert_eq!(v1, v3);
}

#[test]
fn code6() {
    let r = 1..10u8;
    let xs = r.collect::<Vec<u8>>();
    println!("{:?}", xs);
}

#[test]
fn code7() {
    let a = [0u8; 10];
    let it = a.iter();
    let v = it.collect::<Vec<&u8>>(); // FromIterator
}

#[test]
fn code8() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2),
        }
    }
    assert_eq!(v, vec![2, 3, 4, 5, 6]);
}

#[test]
fn code9() {
    let mut v = vec![1, 2, 3];
    let s1 = &v[..];
    let s2 = &v[0..v.len()];
    let vref = &mut v;
    vref.push(4);
    let s3 = &v[0..];
    // println!("{:?}", s1);
    // println!("{:?}", s2);
    // println!("{:?}", s3);
    // println!("{:?}", vref);
}

#[test]
fn grows_twice_starting_from_asked() {
    let mut xs: Vec<char> = Vec::with_capacity(200);
    for _ in 0..1000 {
        xs.push('a');
        println!("capacity:{}", xs.capacity());
    }
    assert_eq!(1000, xs.len());
    assert_eq!(1600, xs.capacity());
}

#[test]
fn code11() {
    // using enums lets us store different things
    let v: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
    ];
}

trait Ip {
    fn display(&self) -> String;
}

struct V4(String);
struct V6(String);

impl Ip for V4 {
    fn display(&self) -> String {
        format!("V4({})", self.0)
    }
}

impl Ip for V6 {
    fn display(&self) -> String {
        format!("V6({})", self.0)
    }
}

#[test]
fn code12_dynamic_dispatch() {
    // using box we can put subtypes of trait
    let v: Vec<Box<dyn Ip>> = vec![Box::new(V4("127.0.0.1".to_string())), Box::new(V6("::1".to_string()))];
    for ip in v {
        println!("{}", ip.display());
    }
}

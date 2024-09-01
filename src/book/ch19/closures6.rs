// fn - is a function pointer

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}
#[test]
fn code1() {
    let mul3 = |x: i32| x * 3;
    let x = do_twice(mul3, 10);
    println!("{}", x); // 10 * 3 * 3 = 90
}

#[test]
fn code2() {
    let bs = vec![1, 2, 3];
    let cs: Vec<String> = bs
        .iter()
        .map(|i| i.to_string())
        .collect();
    dbg!(cs);
}

#[test]
fn code3() {
    let bs = vec![1, 2, 3];
    let cs: Vec<String> = bs
        .iter()
        .map(|i| i.to_string())
        .collect();
    dbg!(cs);
}

fn fmt(x: &i32) -> String {
    x.to_string()
}

#[test]
fn code4() {
    let cs = vec![1, 2, 3]
        .iter()
        .map(fmt)
        .collect::<Vec<String>>();
    dbg!(cs);
}

#[test]
fn code5() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let xs = (0u32..20)
        .map(Status::Value)
        .collect::<Vec<Status>>();

    xs.iter()
        .for_each(|x| println!("{:?}", x))
}

fn inc_fn() -> Box<dyn Fn(u8) -> u8> {
    Box::new(|x| x + 1)
}
#[test]
fn code6() {
    let xs = (0u8..=10) // 0..10
        .map(inc_fn())
        .collect::<Vec<u8>>();
    dbg!(xs); // 1..11
}

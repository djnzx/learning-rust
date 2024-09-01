#[test]
fn code1() {
    // irrefutable pattern
    let x = 5;
}

fn go(ox: Option<u8>) {
    // refutable, will not compile
    // let Some(x) = ox;

    // but this will
    let Some(x) = ox else { panic!() };
}

fn go2(ox: Option<u8>) {
    // here we will not handle None (other branches)
    if let Some(x) = ox {
        todo!()
    }
}

#[test]
fn code2() {
    // compiler complains, but works well
    if let x = 5 {
        println!("{x}");
    };
}

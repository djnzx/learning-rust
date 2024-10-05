// longer than 12 can't be printed
#[test]
fn code1() {
    let x = (1, 2.3);
}

// to print struct we need to derive Debug
#[derive(Debug)]
struct Box {
    x: u32,
    y: u32,
}

// dbg - stderr
// print - stdout
// partial move - original one can't be used

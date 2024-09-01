use std::fmt::Debug;
use std::panic;
use std::thread;

fn div_unsafe(a: u32, b: u32) -> u32 {
    a / b
}

fn div_safe(a: u32, b: u32) -> thread::Result<u32> {
    match panic::catch_unwind(|| div_unsafe(a, b)) {
        x @ Ok(_) => x,
        e @ Err(_) => e,
    }
}

#[test]
pub fn test1() {
    let x: thread::Result<u32> = div_safe(25, 1);
    dbg!(x);
    let x: thread::Result<u32> = div_safe(25, 0);
    match dbg!(x) {
        Ok(x) => {
            println!("good: {x}");
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

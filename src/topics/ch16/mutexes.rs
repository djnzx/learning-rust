use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn code1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // block current thread until allowed
        *num = 6;
    }

    println!("m = {m:?}");
}

// there are simpler things if we need math only
// https://doc.rust-lang.org/std/sync/atomic/index.html
#[test]
fn code2() {
    // mutex wrapped into atomic reference
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // clone counter for each thread
        let counter = Arc::clone(&counter);
        let block = move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        };

        let handle = thread::spawn(block);

        // register handles
        handles.push(handle);
    }

    // join (wait for all)
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); //10
}

#[test]
fn code3() {}

#[test]
fn code4() {}

#[test]
fn code5() {}

#[test]
fn code6() {}

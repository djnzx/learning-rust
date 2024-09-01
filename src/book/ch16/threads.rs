use std::thread;
use std::time::Duration;
#[test]
fn code1() {
    let block = || {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    };

    let handle = thread::spawn(block);

    for i in 1..=3 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // all spawned are terminated when main is completed
    // here we wait for join to be completed
    handle.join().unwrap(); // without that we don't know how many number will be printed in line 6
}

#[test]
fn code2() {
    let v = vec![1, 2, 3];

    //                                       but only one is allowed
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

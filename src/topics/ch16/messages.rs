use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn code1() {
    /// multiple producers, ONE consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("sent: {}", &val);
        tx.send(val).unwrap();
        // no `val` here
    });

    // without sending it will hang forever waiting
    let received: String = rx.recv().unwrap();
    println!("Got: {received}");
}

#[test]
fn code2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receiver is Iterator
    for received in rx {
        println!("Got: {received}");
    }
}

#[test]
fn code3() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1 hi"),
            String::from("1 from"),
            String::from("1 the"),
            String::from("1 thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2 more"),
            String::from("2 messages"),
            String::from("2 for"),
            String::from("2 you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // no ordering but wait all
    for received in rx {
        println!("Got: {received}");
    }
}

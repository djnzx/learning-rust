use std::fmt::{format, Debug, Display};

trait Showable {
    fn show(&self) -> String;
}

struct S1;
struct S2;

impl Showable for S1 {
    fn show(&self) -> String {
        "showable for S1".to_string()
    }
}

impl Showable for S2 {
    fn show(&self) -> String {
        "showable for S2".to_string()
    }
}

fn whatever(x: impl Showable) {}

#[test]
fn test1() {
    whatever(S1);
    whatever(S2);
}

#[test]
fn test2() {
    println!("{}", S1.show());
    println!("{}", S2.show());
}

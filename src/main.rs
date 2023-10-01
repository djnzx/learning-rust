use rand::Rng;
use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::ops::Add;
use topics::ch02;

mod topics;

fn main() {
    ch02::guess_app::main()
}
fn main0() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let matches = re.is_match("2014-01-01");
    println!("Did our date match? {}", matches);
}

fn main3() {
    println!("Hello!")
}

/// main can be Unit
fn main2() {
    topics::topic19iter::main()
}

/// main can be Result (Either)
fn main1() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

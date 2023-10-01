use std::fmt::Display;
use std::ops::Add;
use rand::Rng;
use std::error::Error;
use std::fs::File;

mod topics;

/// main can be Unit
fn main() {
    topics::topic16lifetimes::playground()
}

/// main can be Either
fn main1() -> Result<(), Box<dyn Error>> {
    topics::topic14result::playground();
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

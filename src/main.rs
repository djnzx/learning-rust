use book::ch02;
use rand::Rng;
use std::error::Error;
use std::fmt::Display;
use std::ops::Add;

mod book;
mod usecases;

/// main can be unit
fn main() {
    ch02::guess_app::main()
}

/// main can be Result (Either)
fn main1() -> Result<(), Box<dyn Error>> {
    Ok(())
}

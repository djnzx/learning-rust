use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};

#[test]
fn test1_Ok() {
    let x = File::open("hello.txt");
    let f = match dbg!(x) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("file not found {e:?}"),
            _ => panic!("other error {e}"),
        },
    };
}

#[test]
fn test2_Err() {
    let x: std::io::Result<File> = File::open("hello1.txt");
    match dbg!(x) {
        Ok(f) => {}
        Err(e) => {
            let k = e.kind();
            dbg!(e.kind());
        }
    }
}

fn create_or_panic(file_name: &str) -> File {
    File::create(file_name).unwrap_or_else(|e| {
        panic!("Problem creating the file: {:?}", e);
    })
}

fn open_or_create(file_name: &str) {
    File::open(file_name).unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => create_or_panic(file_name),
        _ => panic!("Other open error {:?}", e),
    });
}

#[test]
fn test2_unwrap() {
    let f = open_or_create("hello1.txt");
}
fn test2_or_panic() {
    // get or throw
    let greeting_file = File::open("hello.txt").unwrap();
    // get or throw with message
    let r = File::open("hello.txt").expect("message");
}

use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::FromStr;

fn read_username_from_file(name: &str) -> Result<String, io::Error> {
    let open_result = File::open(name);

    let mut f = match open_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    match f.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

#[test]
fn test3() {
    let s = read_username_from_file("hello.txt");
    let content = match s {
        Ok(content) => content,
        Err(x) => panic!("{}", x),
    };
    dbg!(content);
}

fn read_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // return error. return type should have result with the same error
    let mut content = String::new();
    f.read_to_string(&mut content)?; // shortcuts to Result<_, io::Error>
    Ok(content)
}

fn read_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn a() -> Option<u8> {
    Some(100)
}

fn b() -> Option<u8> {
    let x = a()?; // shortcut to None or Result.error

    Some(x)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn playground2() {
    // can't use "?", because it should return Result
    // let greeting_file = File::open("hello.txt")?;

    use std::net::IpAddr;

    let home = "127.0.0.1" // TODO: how parse works ??? how to write custom parsers ???
        .parse::<IpAddr>()
        .expect("Hardcoded IP address should be valid");
}

/// data representation
pub struct GuessedValue {
    value: i32,
}

impl GuessedValue {
    /// constructor
    pub fn new(value: i32) -> GuessedValue {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        GuessedValue { value }
    }

    /// getter
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl FromStr for GuessedValue {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<i32>()?;
        Ok(GuessedValue { value })
    }
}

fn playground() {
    let mut guess = String::new();

    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = match guess.trim().parse::<GuessedValue>() {
            Ok(num) => num.value,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {}
            Ordering::Equal => {}
            Ordering::Greater => {}
        }
    }
}

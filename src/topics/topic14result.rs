use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

pub fn playground1() {
    // Ok + file handle
    let file: std::io::Result<File> = File::open("hello.txt");

    match file {
        Ok(f) => {dbg!("ok", f);}
        Err(x) => {dbg!("err", x);}
    }

    // Err + kind NotFound
    let file: std::io::Result<File> = File::open("hello2.txt");

    match file {
        Ok(f) => {dbg!("ok", f);}
        Err(x) => match x.kind() {
            ErrorKind::NotFound => { println!("file not found");}
            other => { dbg!("other error: ", other);}
        }
    }

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error_open| {
        if error_open.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error_create| {
                panic!("Problem creating the file: {:?}{:?}", error_create, error_open);
            })
        } else {
            panic!("Problem opening the file: {:?}", error_open);
        }
    });

    // get or throw
    let greeting_file = File::open("hello.txt").unwrap();
    // get or throw with message
    let r = File::open("hello.txt").expect("message");


}

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // shortcuts to Result<_, io::Error>
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;           // shortcuts to Result<_, io::Error>
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
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
    text
        .lines()
        .next()?
        .chars()
        .last()
}

pub fn playground2() {
    // can't use "?", because it should return Result
    // let greeting_file = File::open("hello.txt")?;

    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1" // TODO: how parse works ??? how to write custom parsers ???
        .parse()
        .expect("Hardcoded IP address should be valid");
}

pub fn playground3() {
    let mut guess = String::new();

    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        /// trivial int parsing
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
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

pub fn playground() {
    let mut guess = String::new();

    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop {
    //     let guess: GuessedValue = match guess.trim().parse() {
    //         Ok(num) => num.value,
    //         Err(_) => continue,
    //     };
    //
    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => {}
    //         Ordering::Equal => {}
    //         Ordering::Greater => {}
    //     }
    // }

}
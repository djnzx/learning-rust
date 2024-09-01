use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    println!("Guess the number!");

    let mx = 128;
    let mn = 0;

    // ..= is alias for RangeInclusive
    let ri = mn..=mx;

    let mut rt = rand::thread_rng();

    let secret_number = rt.gen_range(ri);

    let mut min = mn;
    let mut max = mx;

    loop {
        println!("Please input your guess. between {min} and {max}!");
        let center = (min + max) / 2;
        println!("hint: middle: {center}");

        // without parenthesis is a function unapplied
        let guess_fn = String::new;
        // empty mutable allocation, takes no time, since doesn't actually allocate
        let mut buffer = guess_fn();

        let stdin = io::stdin();

        // read into a buffer
        let read_size = stdin.read_line(&mut buffer);

        // the semantic getOrElse(throw Error), but we don't care about the result
        // we just fail in case of error
        let _ = read_size.expect("Failed to read a line from the console");

        // trimmed without original modification
        let trimmed = buffer.trim();

        if trimmed == "q" {
            println!("Quitting");
            break;
        }

        // parse string as u32
        let parsed = trimmed.parse::<u32>();

        let guess = match parsed {
            Ok(num) => num,
            Err(x) => continue, // if error -> don't care, go to start
        };

        // different ways of passing variables to interpolation
        println!("You guessed: {guess}");
        // println!("You guessed: {}", guess);

        // named u8 with values -1,0,1
        let compared = guess.cmp(&secret_number);

        match compared {
            Ordering::Less => {
                min = guess;
                println!("Too small!")
            } // -1
            Ordering::Greater => {
                max = guess;
                println!("Too big!")
            } // 1
            Ordering::Equal => {
                println!("You won!"); // 0
                break;
            }
        }
    }
}

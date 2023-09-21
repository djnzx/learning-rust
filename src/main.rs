use std::cmp::Ordering;
use std::io;
use ferris_says::say;
use std::io::{stdout, BufWriter, Stdin, Error};
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let mut guess = String::new();

    let in0: Stdin = io::stdin();

    in0
        .read_line(&mut guess)
        .expect("Failed to read line")
    ;

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    println!("You guessed: {guess}");

    let x = match guess.cmp(&guess) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };

    let mut x = 0;

    loop {
        println!("{x}");
        x += 1;
        match x.cmp(&5) {
            Ordering::Less => {}
            Ordering::Equal => {println!("Win!") ; break}
            Ordering::Greater => {}
        }
    }

    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
    }



}

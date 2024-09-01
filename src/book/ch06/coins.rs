#[derive(PartialEq)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    NewJersey,
    Delaware,
}

enum Coin {
    Penny,
    Nickel(u8),
    Dime(String),
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel(1) => 5,
        Coin::Nickel(_) => 5,
        Coin::Dime(x) if x == "1" => 10,
        Coin::Dime(_) => 10,
        Coin::Quarter(UsState::Alabama) => 25,
        Coin::Quarter(state) if state == UsState::NewJersey => 25,
        Coin::Quarter(state) => 25,
    }
}

fn cents_to_coin(x: u8) -> Option<Coin> {
    match x {
        1 => Some(Coin::Penny),
        5 => Some(Coin::Nickel(25)),
        10 => Some(Coin::Dime(String::from("123"))),
        25 => Some(Coin::Quarter(UsState::Alabama)),
        _ => None,
    }
}

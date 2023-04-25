#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn money() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    let alabama_quarter = Coin::Quarter(UsState::Alabama);

    println!("Penny: {}", value_in_cents(penny));
    println!("Nickel: {}", value_in_cents(nickel));
    println!("Dime: {}", value_in_cents(dime));
    println!("Alaska Quarter: {}", value_in_cents(alaska_quarter));
    println!("Alabama Quarter: {}", value_in_cents(alabama_quarter));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
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
            println!("Nice! State is {state:?}");
            25
        }
    }
}

fn plus_one_sometimes(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let my_coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(my_coin);
    println!("{value}");

    let five = Some(5);
    let six = plus_one_sometimes(five);
    let none = plus_one_sometimes(None);
    println!("{six:?}");

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Get fancy hat :)"),
        7 => println!("Lose fancy hat :("),        
        _ => (), // nothing else happens (unit value)
    }

    // using if lets
    let config_max: Option<u8> =  Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
